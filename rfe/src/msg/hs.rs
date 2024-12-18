use crate::utils::PerfData;
use bincode::{Decode, Encode};

extern crate alloc;
use crate as rfe;
#[cfg(feature = "reflect")]
use crate::macros::Reflect;
use alloc::vec::Vec;

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct HsHk {
    pub perf: PerfData,
    pub counter: u32,
    pub cpu_usage: Vec<u8>,
    pub mem_usage: u8,
    pub fs_usage: Vec<u8>,
    pub temps: Vec<i8>,
    pub cmd_counter: u8,
    pub cpu_usage_enabled: bool,
    pub mem_usage_enabled: bool,
    pub fs_usage_enabled: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct HsOutData {
    pub counter: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum HsCmd {
    #[default]
    Noop,
    Reset,
    WatchdogEnableManual(bool),
    WatchdogEnableAuto(bool),
    WatchdogResumeAuto,
}
