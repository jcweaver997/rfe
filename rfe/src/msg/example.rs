extern crate alloc;
#[cfg(feature = "to_csv")]
use crate::to_csv::ToCsv;
use crate::utils::PerfData;
use bincode::{Decode, Encode};
#[cfg(feature = "to_csv")]
use macros::ToCsv;

#[derive(Debug, Default, Clone, Copy, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct ExampleHk {
    pub perf: PerfData,
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
