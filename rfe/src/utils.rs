use crate::time::Timestamp;
use crate::Rfe;
use bincode::{Decode, Encode};

extern crate alloc;
#[cfg(feature = "to_csv")]
use crate::to_csv::ToCsv;
#[cfg(feature = "to_csv")]
use macros::ToCsv;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Encode, Decode)]
#[cfg_attr(feature = "to_csv", derive(ToCsv))]
pub struct PerfData {
    enter_time: Timestamp,
    elapsed: u32,
    rate: u32,
}

impl PerfData {
    pub fn new(rfe: &Rfe) -> Self {
        Self {
            enter_time: rfe.get_met_time(),
            elapsed: 0,
            rate: 0,
        }
    }

    pub fn enter(&mut self, rfe: &Rfe) {
        let time = rfe.get_met_time();
        self.rate = (time - self.enter_time) as u32;
        self.enter_time = time;
    }

    pub fn exit(&mut self, rfe: &Rfe) {
        let time = rfe.get_met_time();
        self.elapsed = (time - self.enter_time) as u32;
    }
}

pub struct ManualAuto<T: Clone + PartialEq> {
    value_auto: T,
    value_manual: T,
    is_manual: bool,
    has_changed: bool,
}

impl<T: Clone + PartialEq> ManualAuto<T> {
    pub fn new(value: T, is_manual: bool) -> Self {
        Self {
            value_auto: value.clone(),
            value_manual: value,
            is_manual,
            has_changed: false,
        }
    }

    pub fn auto_set(&mut self, value: T) {
        if self.value_auto != value && !self.is_manual {
            self.has_changed = true;
        }
        self.value_auto = value;
    }

    pub fn manual_set(&mut self, value: T) {
        if (self.value_manual != value && self.is_manual)
            || (!self.is_manual && self.value_auto != value)
        {
            self.has_changed = true;
        }
        self.is_manual = true;
        self.value_manual = value;
    }

    pub fn resume_auto(&mut self) {
        if self.is_manual && self.value_manual != self.value_auto {
            self.has_changed = true;
        }
        self.is_manual = false;
    }

    pub fn get(&self) -> &T {
        if self.is_manual {
            &self.value_manual
        } else {
            &self.value_auto
        }
    }

    pub fn is_manual(&self) -> bool {
        return self.is_manual;
    }

    /// Checks if the internal value has changed since the last call to has_changed
    pub fn has_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            return true;
        }
        return false;
    }
}
