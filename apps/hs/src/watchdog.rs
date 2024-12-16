pub trait Watchdog {
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_timeout(&mut self, time: i32);
    fn feed(&mut self);
}

#[cfg(all(feature = "std", feature = "nix"))]
mod watchdog_nix {
    use super::Watchdog;
    extern crate std;
    use crate::unwrap_print_err;
    use anyhow::Result;
    use log::*;

    pub struct LinuxWatchdog {
        wd: watchdog_device::Watchdog,
    }

    impl LinuxWatchdog {
        pub fn new() -> Result<Self> {
            let wd = watchdog_device::Watchdog::new()?;
            wd.set_option(&watchdog_device::SetOptionFlags::DisableCard)?;
            Ok(Self { wd })
        }
    }

    impl Watchdog for LinuxWatchdog {
        fn enable(&mut self) {
            unwrap_print_err!(
                self.wd
                    .set_option(&watchdog_device::SetOptionFlags::EnableCard),
                "failed to enable watchdog"
            );
        }

        fn disable(&mut self) {
            unwrap_print_err!(
                self.wd
                    .set_option(&watchdog_device::SetOptionFlags::DisableCard),
                "failed to set watchdog timeout"
            );
        }

        fn set_timeout(&mut self, time: i32) {
            unwrap_print_err!(self.wd.set_timeout(time), "failed to set watchdog timeout");
        }

        fn feed(&mut self) {
            unwrap_print_err!(self.wd.keep_alive(), "failed to feed watchdog");
        }
    }
}

#[cfg(all(feature = "std", feature = "nix"))]
pub use watchdog_nix::*;

pub type WatchdogRef<'a> = Option<&'a mut dyn Watchdog>;

impl<'a> Watchdog for WatchdogRef<'a> {
    fn enable(&mut self) {
        if let Some(wd) = self {
            wd.enable();
        }
    }

    fn disable(&mut self) {
        if let Some(wd) = self {
            wd.disable();
        }
    }

    fn set_timeout(&mut self, time: i32) {
        if let Some(wd) = self {
            wd.set_timeout(time);
        }
    }

    fn feed(&mut self) {
        if let Some(wd) = self {
            wd.feed();
        }
    }
}

#[cfg(feature = "rp2040")]
mod watchdog_rp2040 {
    use rp2040_hal::Watchdog;

    pub struct Rp2040Watchdog {
        wd: Watchdog,
        time: u32,
    }

    impl Rp2040Watchdog {
        pub fn new(wd: Watchdog) -> Self {
            Self {
                wd,
                time: 10 * 1000000,
            }
        }
    }

    impl super::Watchdog for Rp2040Watchdog {
        fn enable(&mut self) {
            self.wd
                .start(rp2040_hal::fugit::Duration::<u32, 1, 1000000>::from_ticks(
                    self.time,
                ));
        }

        fn disable(&mut self) {
            self.wd.disable();
        }

        fn set_timeout(&mut self, time: i32) {
            self.time = time as u32 * 1000000;
        }

        fn feed(&mut self) {
            self.wd.feed();
        }
    }
}

#[cfg(feature = "rp2040")]
pub use watchdog_rp2040::*;
