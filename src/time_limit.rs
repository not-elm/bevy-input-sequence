use bevy::prelude::Resource;
use std::time::Duration;
use bevy::reflect::Reflect;
/// A time limit specified as frame counts or duration.
#[derive(Clone, Resource, Debug, Reflect)]
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
