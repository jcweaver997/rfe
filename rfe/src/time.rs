pub type Timestamp = u64;

#[derive(Debug, Clone, Copy)]
pub struct TimeData {
    pub sch_counter: u64,
    pub time_offset: u64,
}

pub trait TimeDriver {
    /// Time in microseconds relative to system epoch
    fn get_system_time(&self, time_data: TimeData) -> Timestamp;
}

#[cfg(feature = "std")]
pub struct UnixTimeDriver;
#[cfg(feature = "std")]
impl UnixTimeDriver {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(feature = "std")]
impl TimeDriver for UnixTimeDriver {
    fn get_system_time(&self, _time_data: TimeData) -> Timestamp {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64
    }
}

pub struct SchTimeDriver;
impl SchTimeDriver {
    pub fn new() -> Self {
        Self {}
    }
}

impl TimeDriver for SchTimeDriver {
    fn get_system_time(&self, time_data: TimeData) -> Timestamp {
        time_data.sch_counter + time_data.time_offset
    }
}
