use bevy::prelude::{Component, Event};

use crate::act::Act;
use crate::key_sequence::KeySequence;
use crate::prelude::Timeout;

#[derive(Component)]
pub struct InputSequence<E>(KeySequence<E>);


impl<E: Event + Clone> InputSequence<E> {

    #[inline(always)]
    pub fn new<T>(
        event: E,
        timeout: Timeout,
        inputs: impl IntoIterator<Item = T>,
    ) -> InputSequence<E> where T: Into<Act> {
        Self(KeySequence::new(event, inputs.into_iter().map(|x| x.into()), timeout))
    }


    #[inline(always)]
    pub(crate) fn event(&self) -> E {
        self.0.event()
    }


    #[inline(always)]
    pub(crate) fn next_input(&self) -> Option<Act> {
        self.0.next_input()
    }


    #[inline(always)]
    pub(crate) fn next_sequence(&self) -> KeySequence<E> {
        self.0.next_sequence()
    }


    #[inline(always)]
    pub(crate) fn once_key(&self) -> bool {
        self.0.is_last()
    }
}


