extern crate alloc;
use crate as rfe;
#[cfg(feature = "reflect")]
use crate::macros::Reflect;
use crate::utils::PerfData;
use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ExampleHk {
    pub perf: PerfData,
    pub counter: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct ExampleOutData {
    pub counter: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum ExampleCmd {
    #[default]
    Noop,
    Reset,
}
