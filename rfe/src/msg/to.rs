#[cfg(feature = "to_csv")]
use crate::to_csv::ToCsv;
use crate::utils::PerfData;
use bincode::{Decode, Encode};
#[cfg(feature = "to_csv")]
use macros::ToCsv;
extern crate alloc;
use alloc::vec::Vec;

use super::{TlmSetId, TlmSetItem};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ToTlmSet {
    pub items: Vec<TlmSetItem>,
    pub id: TlmSetId,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ToHk {
    pub perf: PerfData,
    pub counter: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ToOutData {
    pub counter: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub enum ToCmd {
    Noop,
    Reset,
    AddTlmSet(ToTlmSet),
    RemoveTlmSet(TlmSetId),
    DisableTlmSet(TlmSetId),
    EnablTlmSet(TlmSetId),
}
