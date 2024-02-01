use bevy::prelude::{Component, Event, Time};

use crate::act::Act;
use crate::input_sequence::InputSequence;

#[derive(Component)]
pub(crate) struct SequenceReader<E>(Option<InputSequence<E>>, usize);

impl<E: Event + Clone> SequenceReader<E> {

    #[inline(always)]
    pub(crate) fn new(
        seq: InputSequence<E>,
        start_index: usize
    ) -> SequenceReader<E> {
        Self(Some(seq), start_index)
    }

    #[inline(always)]
    pub(crate) fn event(&mut self) -> E {
        self.0.take().unwrap().event
    }


    #[inline(always)]
    pub(crate) fn next_input(&self) -> Option<&Act> {
        self.0.as_ref().unwrap().inputs.get(self.1)
    }


    #[inline(always)]
    pub(crate) fn next_sequence(&mut self) {
        self.1 += 1;
    }

    #[inline(always)]
    pub(crate) fn is_last(&self) -> bool {
        self.1 >= self.0.as_ref().unwrap().inputs.len()
    }

    #[inline(always)]
    pub(crate) fn timedout(&mut self, time: &Time) -> bool {
        self.0.as_mut().unwrap().timedout(time)
    }


}
