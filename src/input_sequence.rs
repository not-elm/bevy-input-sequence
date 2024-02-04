use bevy::prelude::{Component, Event};

use crate::act::Act;
use crate::timeout::TimeLimit;
use crate::SequenceReader;

/// An input sequence fires an event when its acts are matched within the
/// given time limit.
#[derive(Component, Debug, Clone)]
pub struct InputSequence<E> {
    pub event: E,
    pub time_limit: Option<TimeLimit>,
    pub acts: Vec<Act>,
}

impl<E> InputSequence<E>
where
    E: Event + Clone,
{
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

    /// Return true if there is only one act in the sequence.
    #[inline(always)]
    pub(crate) fn one_key(&self) -> bool {
        1 == self.acts.len()
    }

    /// Return the first act or input.
    pub fn first_input(&self) -> Option<&Act> {
        self.acts.get(0)
    }

    #[inline(always)]
    pub(crate) fn start_reader(self, at: usize) -> SequenceReader<E> {
        SequenceReader::new(self, at)
    }
}
