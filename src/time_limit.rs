use bevy::reflect::Reflect;
use std::time::Duration;
/// A time limit specified as frame counts or duration.
#[derive(Clone, Debug, Reflect)]
pub enum TimeLimit {
    /// Time limit for frame count
    Frames(u32),
    /// Time limit for duration
    Duration(Duration),
}

impl From<Duration> for TimeLimit {
    #[inline(always)]
    fn from(duration: Duration) -> Self {
        Self::Duration(duration)
    }
}
