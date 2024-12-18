use crate::utils::PerfData;
use bincode::{Decode, Encode};
extern crate alloc;
use crate as rfe;
#[cfg(feature = "reflect")]
use crate::macros::Reflect;
use alloc::vec::Vec;

use super::{TlmSetId, TlmSetItem};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ToTlmSet {
    pub items: Vec<TlmSetItem>,
    pub id: TlmSetId,
    pub enabled: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ToHk {
    pub perf: PerfData,
    pub counter: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ToOutData {
    pub counter: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum ToCmd {
    #[default]
    Noop,
    Reset,
    AddTlmSet(ToTlmSet),
    RemoveTlmSet(TlmSetId),
    DisableTlmSet(TlmSetId),
    EnablTlmSet(TlmSetId),
}
