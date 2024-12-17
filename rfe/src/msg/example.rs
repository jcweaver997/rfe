extern crate alloc;
use crate::utils::PerfData;
use bincode::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExampleHk {
    pub perf: PerfData,
    pub counter: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExampleOutData {
    pub counter: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExampleCmd {
    #[default]
    Noop,
    Reset,
}
