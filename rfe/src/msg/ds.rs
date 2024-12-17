#[cfg(feature = "to_csv")]
use crate::to_csv::ToCsv;
use crate::utils::PerfData;
use bincode::{Decode, Encode};
#[cfg(feature = "to_csv")]
use macros::ToCsv;
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::{TlmSetId, TlmSetItem};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct DsTlmSet {
    pub items: Vec<TlmSetItem>,
    pub id: TlmSetId,
    pub enabled: bool,
    pub path: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct DsHk {
    pub perf: PerfData,
    pub counter: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct DsOutData {
    pub counter: u32,
    pub bytes_written: u32,
    pub bytes_written_this_cycle: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub enum DsCmd {
    Noop,
    Reset,
    CloseAll,
    Close(TlmSetId),
    AddTlmSet(DsTlmSet),
    RemoveTlmSet(TlmSetId),
    DisableTlmSet(TlmSetId),
    EnablTlmSet(TlmSetId),
}
