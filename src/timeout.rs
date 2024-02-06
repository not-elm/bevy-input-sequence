use std::time::Duration;

use bevy::prelude::{Resource, TimerMode};
use bevy::time::{Time, Timer};

/// Specify a time limit in either frame counts or duration.
#[derive(Clone, Resource, Debug)]
pub enum TimeLimit {
    /// Time limit for frame count
    Frames(u32),
    /// Time limit for duration
    Duration(Duration),
}

/// Tracks a time limit at runtime.
#[derive(Resource, Debug)]
pub(crate) enum Timeout {
    None,
    Frames { limit: u32, current: u32 },
    Time(Timer),
}

impl Timeout {
    #[inline(always)]
    pub(crate) fn timedout(&mut self, time: &Time) -> bool {
        match self {
            Self::None => false,
            Self::Time(timer) => timer.tick(time.delta()).finished(),
            Self::Frames { limit, current } => {
                *current += 1;
                limit <= current
            }
        }
    }
}

impl From<Duration> for TimeLimit {
    #[inline(always)]
    fn from(duration: Duration) -> Self {
        Self::Duration(duration)
    }
}

impl From<TimeLimit> for Timeout {
    #[inline(always)]
    fn from(time_limit: TimeLimit) -> Self {
        match time_limit {
            TimeLimit::Frames(frames) => Timeout::Frames {
                limit: frames,
                current: 0,
            },
            TimeLimit::Duration(duration) => Self::Time(Timer::new(duration, TimerMode::Once)),
        }
    }
}

impl From<Option<TimeLimit>> for Timeout {
    #[inline(always)]
    fn from(time_limit: Option<TimeLimit>) -> Self {
        time_limit
            .map(|limit| limit.into())
            .unwrap_or(Timeout::None)
    }
}
