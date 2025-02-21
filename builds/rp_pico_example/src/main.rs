#![no_std]
#![no_main]

use rtic_monotonics::rp2040::prelude::*;

rp2040_timer_monotonic!(Mono);

#[rtic::app(device = rp_pico::hal::pac, dispatchers = [SW0_IRQ])]
mod app {
    use super::*;

    use embassy_rp::{
        interrupt::typelevel,
        peripherals::USB,
        usb::{self, Driver},
    };
    use embedded_hal::digital::v2::OutputPin;
    use example::Example;
    use fugit::Duration;
    use hs::{Hs, HsConfig, Rp2040Watchdog, StubSystemInfoGrabber};
    use log::info;
    use msg::{Instance, MsgKind, MsgPacket, TargetMsg, TlmSetItem, ToTlmSet};
    use rfe::{connector::Connector, Rate, *};
    use rp_pico::hal::{
        clocks,
        gpio::{bank0::Gpio25, FunctionSio, Pin, PullNone, SioOutput},
        Sio, Watchdog,
    };

    use anyhow::Result;
    use core::{mem::MaybeUninit, ptr::addr_of_mut};
    use embedded_alloc::LlffHeap as Heap;
    use panic_halt as _;
    use rp_pico::XOSC_CRYSTAL_FREQ;
    use time::Rp2040TimeDriver;
    extern crate alloc;
    use alloc::vec;
    use alloc::vec::Vec;
    use hashbrown::HashMap;
    use to::*;

    #[global_allocator]
    static HEAP: Heap = Heap::empty();

    /// will log messages to log
    #[derive(Debug)]
    struct LogConnector;

    impl Connector for LogConnector {
        fn send(&mut self, msgs: Vec<MsgPacket>) {
            info!("got msgs: {:?}", msgs);
        }
        fn recv(&mut self) -> Option<Vec<MsgPacket>> {
            return None;
        }
    }

    struct BlinkApp<'a> {
        led_pin: &'a mut Pin<Gpio25, FunctionSio<SioOutput>, PullNone>,
        on: bool,
        counter: u32,
    }

    impl App for BlinkApp<'_> {
        fn init(&mut self, _rfe: &mut Rfe) -> Result<()> {
            return Ok(());
        }

        fn run(&mut self, _rfe: &mut Rfe) {
            self.counter += 1;
            self.on = !self.on;
            if self.on {
                self.led_pin.set_high().ok();
            } else {
                self.led_pin.set_low().ok();
            }
        }

        fn hk(&mut self, _rfe: &mut Rfe) {}

        fn out_data(&mut self, _rfe: &mut Rfe) {}

        fn get_app_rate(&self) -> Rate {
            return Rate::Hz10;
        }
    }

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<Gpio25, FunctionSio<SioOutput>, PullNone>,
        usb: Option<Driver<'static, USB>>,
        wd: Option<Watchdog>,
        time_driver: Option<Rp2040TimeDriver>,
    }

    struct UsbBinding;
    unsafe impl typelevel::Binding<typelevel::USBCTRL_IRQ, usb::InterruptHandler<USB>> for UsbBinding {}

    #[init()]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        let p = embassy_rp::init(Default::default());
        // Configure the clocks, watchdog - The default is to generate a 125 MHz system clock

        let mut watchdog = Watchdog::new(ctx.device.WATCHDOG);
        let clocks = clocks::init_clocks_and_plls(
            XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();
        let time_driver = Rp2040TimeDriver::new(ctx.device.TIMER, &mut ctx.device.RESETS, &clocks);
        Mono::start(
            unsafe { rp_pico::pac::TIMER::steal() },
            &mut ctx.device.RESETS,
        ); // default rp2040 clock-rate is 125MHz

        {
            const HEAP_SIZE: usize = 1024 * 16;
            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
        }

        let driver = Driver::new(p.USB, UsbBinding {});
        let sio = Sio::new(ctx.device.SIO);
        let gpioa = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );
        let led = gpioa
            .led
            .into_pull_type::<PullNone>()
            .into_push_pull_output();

        // Spawn heartbeat task
        blink_instance::spawn().ok();
        usb_logger::spawn().ok();

        // Return resources and timer
        (
            Shared {},
            Local {
                led,
                usb: Some(driver),
                wd: Some(watchdog),
                time_driver: Some(time_driver),
            },
        )
    }

    #[task(local = [usb])]
    async fn usb_logger(ctx: usb_logger::Context) {
        let driver = Option::take(ctx.local.usb).unwrap();
        embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
    }

    #[task(binds = USBCTRL_IRQ)]
    fn usbctrl(_cx: usbctrl::Context) {
        unsafe {
            <usb::InterruptHandler<USB> as typelevel::Handler<typelevel::USBCTRL_IRQ>>::on_interrupt();
        }
    }

    #[task(local = [led, wd, time_driver], priority = 1)]
    async fn blink_instance(mut ctx: blink_instance::Context) {
        let mut blink_app = BlinkApp {
            led_pin: &mut ctx.local.led,
            on: false,
            counter: 0,
        };

        let mut log_connector = LogConnector {};
        let mut tlmsets = HashMap::new();
        tlmsets.insert(
            0,
            ToTlmSet {
                items: vec![TlmSetItem {
                    target: TargetMsg::new(Instance::All, MsgKind::ExampleHk),
                    counter: 0,
                    decimation: 0,
                }],
                id: 0,
                enabled: true,
            },
        );
        let mut to = To::new(&mut log_connector, tlmsets);
        let mut example = Example::new();
        let mut wd = Rp2040Watchdog::new(ctx.local.wd.take().unwrap());

        let mut grabber = StubSystemInfoGrabber::new();
        let mut hs = Hs::new(
            HsConfig {
                cpu_checks: false,
                fs_checks: false,
                mem_checks: false,
                temp_checks: false,
                watchdog_enable: true,
                watchdog_timeout: 3,
            },
            &mut grabber,
            Some(&mut wd),
        );
        let time_driver = ctx.local.time_driver.take().unwrap();

        let mut instance = RfeInstance::new(Instance::Example, &time_driver);
        instance.add_app("blink_app", &mut blink_app).unwrap();
        instance.add_app("to", &mut to).unwrap();
        instance.add_app("hs", &mut hs).unwrap();
        instance.add_app("example", &mut example).unwrap();

        let mut next_time = Mono::now() + Duration::<u64, 1, 1000000>::from_ticks(10000);

        loop {
            instance.run();
            Mono::delay_until(next_time).await;
            next_time += Duration::<u64, 1, 1000000>::from_ticks(10000);
        }
    }
}
