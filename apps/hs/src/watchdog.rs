pub trait Watchdog {
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_time(&mut self, time: i32);
    fn feed(&mut self);
}

#[cfg(feature = "std")]
mod watchdog_std {
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

        fn set_time(&mut self, time: i32) {
            unwrap_print_err!(self.wd.set_timeout(time), "failed to set watchdog timeout");
        }

        fn feed(&mut self) {
            unwrap_print_err!(self.wd.keep_alive(), "failed to feed watchdog");
        }
    }
}

#[cfg(feature = "std")]
pub use watchdog_std::*;

pub type WatchdogRef<'a> = Option<&'a mut dyn Watchdog>;

impl<'a> Watchdog for Option<&'a mut dyn Watchdog> {
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

    fn set_time(&mut self, time: i32) {
        if let Some(wd) = self {
            wd.set_time(time);
        }
    }

    fn feed(&mut self) {
        if let Some(wd) = self {
            wd.feed();
        }
    }
}
