/// Microseconds timestamp
pub type Timestamp = u64;

#[derive(Debug, Clone, Copy)]
pub struct TimeData {
    pub sch_counter: u64,
    pub time_offset: Timestamp,
}

pub trait TimeDriver {
    /// Time in microseconds relative to system epoch
    fn get_system_time(&self, time_data: TimeData) -> Timestamp;

    /// Time in microseconds since program start or power on
    fn get_monotonic_time(&self, time_data: TimeData) -> Timestamp;
}

#[cfg(feature = "std")]
pub struct UnixTimeDriver {
    start_time: std::time::Instant,
}
#[cfg(feature = "std")]
impl UnixTimeDriver {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
}

#[cfg(feature = "std")]
impl TimeDriver for UnixTimeDriver {
    fn get_system_time(&self, _time_data: TimeData) -> Timestamp {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros() as Timestamp
    }

    fn get_monotonic_time(&self, _time_data: TimeData) -> Timestamp {
        self.start_time.elapsed().as_micros() as Timestamp
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

    fn get_monotonic_time(&self, time_data: TimeData) -> Timestamp {
        return time_data.sch_counter * 10000;
    }
}

#[cfg(feature = "rp2040")]
pub struct Rp2040TimeDriver {
    timer: rp2040_hal::timer::Timer,
}

#[cfg(feature = "rp2040")]
impl Rp2040TimeDriver {
    pub fn new(
        timer: rp2040_pac::TIMER,
        resets: &mut rp2040_pac::RESETS,
        clocks: &rp2040_hal::clocks::ClocksManager,
    ) -> Self {
        Self {
            timer: rp2040_hal::timer::Timer::new(timer, resets, clocks),
        }
    }
}

#[cfg(feature = "rp2040")]
impl TimeDriver for Rp2040TimeDriver {
    fn get_system_time(&self, time_data: TimeData) -> Timestamp {
        self.timer.get_counter().ticks() + time_data.time_offset
    }

    fn get_monotonic_time(&self, _time_data: TimeData) -> Timestamp {
        self.timer.get_counter().ticks()
    }
}
