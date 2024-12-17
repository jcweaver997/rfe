#[cfg(feature = "to_csv")]
use crate::to_csv::ToCsv;
use crate::utils::PerfData;
use bincode::{Decode, Encode};
#[cfg(feature = "to_csv")]
use macros::ToCsv;
extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
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
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct HsOutData {
    pub counter: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub enum HsCmd {
    Noop,
    Reset,
    WatchdogEnableManual(bool),
    WatchdogEnableAuto(bool),
    WatchdogResumeAuto,
}
