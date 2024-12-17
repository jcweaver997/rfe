use crate::utils::PerfData;
use bincode::{Decode, Encode};
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use super::{TlmSetId, TlmSetItem};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DsTlmSet {
    pub items: Vec<TlmSetItem>,
    pub id: TlmSetId,
    pub enabled: bool,
    pub path: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DsHk {
    pub perf: PerfData,
    pub counter: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DsOutData {
    pub counter: u32,
    pub bytes_written: u32,
    pub bytes_written_this_cycle: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DsCmd {
    #[default]
    Noop,
    Reset,
    CloseAll,
    Close(TlmSetId),
    AddTlmSet(DsTlmSet),
    RemoveTlmSet(TlmSetId),
    DisableTlmSet(TlmSetId),
    EnablTlmSet(TlmSetId),
}
