extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use bincode::{Decode, Encode};
use macros::Kind;

mod example;
pub use example::*;
mod hs;
pub use hs::*;
mod to;
pub use to::*;
mod ds;
pub use ds::*;

use crate::time::Timestamp;
#[cfg(feature = "to_csv")]
use crate::to_csv::ToCsv;
#[cfg(feature = "to_csv")]
use macros::ToCsv;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct TargetMsg {
    // instance is FROM for tlm, is TO for cmds
    pub instance: Instance,
    pub msg: MsgKind,
}

impl TargetMsg {
    pub fn new(instance: Instance, msg: MsgKind) -> Self {
        Self { instance, msg }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub enum Instance {
    All,
    Other,
    Example,
    Example2,
}

#[derive(Debug, Clone, PartialEq, Kind, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub enum Msg {
    None,
    SubRequest,
    SubList(SubList),
    SetTimeCmd(u64),
    ReinitApp(ReinitAppCmd),
    ExampleHk(ExampleHk),
    ExampleOutData(ExampleOutData),
    ExampleCmd(ExampleCmd),
    DsHk(DsHk),
    DsOutData(DsOutData),
    DsCmd(DsCmd),
    HsHk(HsHk),
    HsOutData(HsOutData),
    HsCmd(HsCmd),
    ToHk(ToHk),
    ToOutData(ToOutData),
    ToCmd(ToCmd),
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct MsgPacket {
    pub instance: Instance,
    pub msg: Msg,
    pub timestamp: Timestamp,
}

impl MsgPacket {
    pub fn to_target(&self) -> TargetMsg {
        TargetMsg::new(self.instance, self.msg.kind())
    }

    pub fn new(instance: Instance, msg: Msg, timestamp: Timestamp) -> Self {
        Self {
            instance,
            msg,
            timestamp,
        }
    }

    pub fn timestamp(&mut self, timestamp: Timestamp) {
        self.timestamp = timestamp;
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct SubList {
    pub subs: Vec<TargetMsg>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ReinitAppCmd {
    app_name: String,
}

pub type TlmSetId = u16;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct TlmSetItem {
    pub target: TargetMsg,
    pub decimation: u16,
    pub counter: u16,
}
