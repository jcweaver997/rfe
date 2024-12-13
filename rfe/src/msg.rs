extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use bincode::{Decode, Encode};
use macros::Kind;

use crate::time::Timestamp;
#[cfg(feature = "to_csv")]
use crate::ToCsv;
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

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ReinitAppCmd {
    app_name: String,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ExampleHk {
    pub counter: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ExampleOutData {
    pub counter: u32,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub enum ExampleCmd {
    Noop,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct DsTlmSet {
    pub items: Vec<TlmSetItem>,
    pub id: TlmSetId,
    pub enabled: bool,
    pub path: String,
}

pub type TlmSetId = u16;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct TlmSetItem {
    pub target: TargetMsg,
    pub decimation: u16,
    pub counter: u16,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]

pub struct DsHk {
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

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct HsHk {
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
