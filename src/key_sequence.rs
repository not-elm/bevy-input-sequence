use std::collections::VecDeque;

use bevy::prelude::{Component, Event};
use bevy::time::Time;

use crate::act::Act;
use crate::prelude::Timeout;

#[derive(Component, Debug)]
pub(crate) struct KeySequence<E> {
    event: E,
    timeout: Timeout,
    inputs: VecDeque<Act>,
}


impl<E> KeySequence<E>
    where E: Event + Clone
{
    #[inline(always)]
    pub fn new(event: E, inputs: impl IntoIterator<Item = Act>, timeout: Timeout) -> KeySequence<E> {
        Self {
            event,
            timeout,
            inputs: VecDeque::from_iter(inputs)
        }
    }


    #[inline(always)]
    pub fn is_last(&self) -> bool {
        self.inputs.len() <= 1
    }


    #[inline(always)]
    pub fn next_input(&self) -> Option<Act> {
        self.inputs.front().cloned()
    }


    #[inline(always)]
    pub fn event(&self) -> E {
        self.event.clone()
    }


    #[inline(always)]
    pub fn timeout(&mut self, time: &Time) -> bool {
        self.timeout.timeout(time)
    }


    #[inline(always)]
    pub fn next_sequence(&self) -> KeySequence<E> {
        KeySequence {
            event: self.event.clone(),
            timeout: self.timeout.clone(),
            inputs: self.inputs.range(1..).cloned().collect(),
        }
    }
}
