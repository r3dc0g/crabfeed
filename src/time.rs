use std::time::{Duration, SystemTime};

pub type Tick = u128;

pub const TICK_RATE: f64 = 30.0; // 30 ticks per millisecond
pub const TIME_STEP: Duration = Duration::from_millis((1000.0 / TICK_RATE) as u64);
pub const TIME_STEP_MILLIS: Tick = (1000.0 / TICK_RATE) as Tick;

pub trait SystemTimeTick {
    fn now() -> Self;
    fn from_system_time(time: SystemTime) -> Self;
    fn as_system_time(&self) -> SystemTime;
}

impl SystemTimeTick for Tick {
    fn now() -> Self {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    fn from_system_time(time: SystemTime) -> Self {
        time.duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
    }

    fn as_system_time(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(*self as u64)
    }
}
