extern crate alloc;
use alloc::vec::Vec;

pub trait SystemInfoGrabber {
    fn check_cpu_usage(&mut self) -> Vec<u8>;
    fn check_mem_usage(&mut self) -> u8;
    fn check_fs_usage(&mut self) -> Vec<u8>;
    fn check_temps(&mut self) -> Vec<i8>;
}

#[cfg(any(feature = "std"))]
mod infograbber_std {
    use sysinfo::{Components, CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

    use crate::SystemInfoGrabber;
    extern crate alloc;
    use alloc::vec::Vec;

    pub struct StdSystemInfoGrabber {
        system: System,
    }

    impl StdSystemInfoGrabber {
        pub fn new() -> Self {
            Self {
                system: System::new_with_specifics(
                    RefreshKind::new()
                        .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                        .with_memory(MemoryRefreshKind::new().with_ram()),
                ),
            }
        }
    }

    impl SystemInfoGrabber for StdSystemInfoGrabber {
        fn check_cpu_usage(&mut self) -> Vec<u8> {
            self.system.refresh_cpu_usage();
            self.system
                .cpus()
                .iter()
                .map(|x| x.cpu_usage().round() as u8)
                .collect()
        }

        fn check_mem_usage(&mut self) -> u8 {
            self.system.refresh_memory();
            (self.system.used_memory() * 100 / self.system.total_memory()) as u8
        }

        fn check_fs_usage(&mut self) -> Vec<u8> {
            let mut disks = Disks::new_with_refreshed_list();
            let disks = disks.list_mut();
            disks.sort_by(|x, y| x.name().cmp(y.name()));

            disks
                .iter()
                .map(|x| ((x.total_space() - x.available_space()) * 100 / x.total_space()) as u8)
                .collect()
        }

        fn check_temps(&mut self) -> Vec<i8> {
            let mut components = Components::new_with_refreshed_list();
            let components = components.list_mut();
            components.sort_by(|x, y| x.label().cmp(y.label()));

            components
                .iter()
                .map(|x| x.temperature().round() as i8)
                .collect()
        }
    }
}

#[cfg(any(feature = "std"))]
pub use infograbber_std::*;

pub struct StubSystemInfoGrabber;
impl StubSystemInfoGrabber {
    pub fn new() -> Self {
        Self {}
    }
}

impl SystemInfoGrabber for StubSystemInfoGrabber {
    fn check_cpu_usage(&mut self) -> Vec<u8> {
        Vec::new()
    }

    fn check_mem_usage(&mut self) -> u8 {
        255
    }

    fn check_fs_usage(&mut self) -> Vec<u8> {
        Vec::new()
    }

    fn check_temps(&mut self) -> Vec<i8> {
        Vec::new()
    }
}
