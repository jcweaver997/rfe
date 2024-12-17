#![no_std]
use anyhow::Result;
use log::*;
use msg::{HsCmd, HsHk, HsOutData, Msg, MsgKind, TargetMsg};
use rfe::*;

extern crate alloc;

mod watchdog;
use utils::ManualAuto;
pub use watchdog::*;

mod infograbber;
pub use infograbber::*;

#[derive(Debug, Default, Clone)]
pub struct HsData {
    out_data: HsOutData,
    hk: HsHk,
}

#[derive(Debug, Clone, Copy)]
pub struct HsConfig {
    pub cpu_checks: bool,
    pub mem_checks: bool,
    pub fs_checks: bool,
    pub temp_checks: bool,
    pub watchdog_enable: bool,
    pub watchdog_timeout: i32,
}

pub struct Hs<'a> {
    data: HsData,
    config: HsConfig,
    grabber: &'a mut dyn SystemInfoGrabber,
    wd: WatchdogRef<'a>,
    wd_value: ManualAuto<bool>,
}

impl<'a> Hs<'a> {
    pub fn new(
        config: HsConfig,
        grabber: &'a mut dyn SystemInfoGrabber,
        mut watchdog: WatchdogRef<'a>,
    ) -> Self {
        watchdog.set_timeout(config.watchdog_timeout);
        if config.watchdog_enable {
            watchdog.enable();
        } else {
            watchdog.disable();
        }
        Self {
            data: Default::default(),
            config,
            grabber,
            wd: watchdog,
            wd_value: ManualAuto::new(config.watchdog_enable, false),
        }
    }

    fn reset(&mut self) {
        self.data = Default::default();
    }
}

impl App for Hs<'_> {
    fn init(&mut self, rfe: &mut Rfe) -> Result<()> {
        self.reset();
        rfe.subscribe(TargetMsg::new(rfe.get_instance(), MsgKind::HsCmd));

        return Ok(());
    }

    fn run(&mut self, rfe: &mut Rfe) {
        self.data.hk.perf.enter(rfe);
        self.data.out_data.counter += 1;
        while let Some(msg) = rfe.recv() {
            match msg.msg {
                Msg::HsCmd(cmd) => {
                    self.data.hk.cmd_counter += 1;
                    match cmd {
                        HsCmd::Noop => {
                            info!("Noop command received");
                        }
                        HsCmd::Reset => {
                            info!("Reset command received");
                            self.data = Default::default();
                        }
                        HsCmd::WatchdogEnableManual(v) => self.wd_value.manual_set(v),
                        HsCmd::WatchdogEnableAuto(v) => self.wd_value.auto_set(v),
                        HsCmd::WatchdogResumeAuto => self.wd_value.resume_auto(),
                    }
                }
                _ => {
                    warn!(
                        "HS received unexpected message: {:?} from {:?}",
                        msg.msg.kind(),
                        msg.instance
                    );
                }
            }
        }

        if self.wd_value.has_changed() {
            if *self.wd_value.get() {
                self.wd.enable();
            } else {
                self.wd.disable();
            }
        }
        self.wd.feed();

        if self.config.cpu_checks {
            self.data.hk.cpu_usage = self.grabber.check_cpu_usage();
        }
        if self.config.mem_checks {
            self.data.hk.mem_usage = self.grabber.check_mem_usage();
        }
        if self.config.fs_checks {
            self.data.hk.fs_usage = self.grabber.check_fs_usage();
        }
        if self.config.temp_checks {
            self.data.hk.temps = self.grabber.check_temps();
        }

        self.data.hk.perf.exit(rfe);
    }

    fn hk(&mut self, rfe: &mut Rfe) {
        self.data.hk.counter = self.data.out_data.counter;

        rfe.send(Msg::HsHk(self.data.hk.clone()));
    }

    fn out_data(&mut self, rfe: &mut Rfe) {
        rfe.send(Msg::HsOutData(self.data.out_data));
    }

    fn get_app_rate(&self) -> Rate {
        Rate::Hz1
    }
}
