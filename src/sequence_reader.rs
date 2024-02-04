use bevy::prelude::{Component, Event, Time};

use crate::act::Act;
use crate::input_sequence::InputSequence;

#[derive(Component)]
pub(crate) struct SequenceReader<E>(Option<InputSequence<E>>, usize);

impl<E: Event + Clone> SequenceReader<E> {
    #[inline(always)]
    pub(crate) fn new(seq: InputSequence<E>, start_index: usize) -> SequenceReader<E> {
        Self(Some(seq), start_index)
    }

    /// Returns the event. Repeated calls to `event()` will panic.
    #[inline(always)]
    pub(crate) fn event(&mut self) -> E {
        self.0.take().expect("No input sequence in reader").event
    }

    #[inline(always)]
    pub(crate) fn next_input(&self) -> Option<&Act> {
        self.0.as_ref().and_then(|x| x.inputs.get(self.1))
    }

    #[inline(always)]
    pub(crate) fn next_act(&mut self) {
        self.1 += 1;
    }

    #[inline(always)]
    pub(crate) fn is_last(&self) -> bool {
        self.0
            .as_ref()
            .map(|x| self.1 >= x.inputs.len())
            .unwrap_or(true)
    }

    #[inline(always)]
    pub(crate) fn timedout(&mut self, time: &Time) -> bool {
        self.0.as_mut().map(|x| x.timedout(time)).unwrap_or(false)
    }
}
