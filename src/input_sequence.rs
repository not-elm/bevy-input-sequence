use bevy::prelude::{Component, Event};

use crate::act::Act;
use crate::time_limit::TimeLimit;

/// An input sequence is a series of [Act]s that fires an event when matched
/// with inputs within the given time limit.
#[derive(Component, Debug, Clone)]
pub struct InputSequence<E> {
    /// Event emitted
    pub event: E,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<Act>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
}

impl<E> InputSequence<E>
where
    E: Event + Clone,
{
    /// Create new input sequence. Not operant until added to an entity.
    #[inline(always)]
    pub fn new<T>(event: E, acts: impl IntoIterator<Item = T>) -> InputSequence<E>
    where
        Act: From<T>,
    {
        Self {
            event,
            time_limit: None,
            acts: Vec::from_iter(acts.into_iter().map(Act::from)),
        }
    }

    /// Specify a time limit from the start of the first matching input.
    pub fn time_limit(mut self, time_limit: impl Into<TimeLimit>) -> Self {
        self.time_limit = Some(time_limit.into());
        self
    }
}
