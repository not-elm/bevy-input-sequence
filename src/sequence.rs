use std::collections::VecDeque;

use bevy::prelude::{Component, Event, GamepadAxisType, GamepadButtonType, KeyCode};
use bevy::time::Time;

use crate::input::Entry;
use crate::prelude::Timeout;

#[derive(Component, Debug)]
pub(crate) struct KeySequence<E> {
    event: E,
    timeout: Timeout,
    inputs: VecDeque<Entry>,
}


impl<E> KeySequence<E>
    where E: Event + Clone
{
    #[inline(always)]
    pub fn from_keycodes(event: E, timeout: Timeout, keycodes: &[KeyCode]) -> KeySequence<E> {
        Self {
            event,
            timeout,
            inputs: VecDeque::from_iter(keycodes.iter().copied().map(Entry::Key)),
        }
    }


    #[inline(always)]
    pub fn from_pad_buttons(event: E, timeout: Timeout, buttons: &[GamepadButtonType]) -> KeySequence<E> {
        Self {
            event,
            timeout,
            inputs: VecDeque::from_iter(buttons.iter().copied().map(Entry::PadButton)),
        }
    }


    #[inline(always)]
    pub fn from_pad_button_axes(event: E, timeout: Timeout, axes: &[GamepadButtonType]) -> KeySequence<E> {
        Self {
            event,
            timeout,
            inputs: VecDeque::from_iter(axes.iter().copied().map(Entry::PadButtonAxis)),
        }
    }


    #[inline(always)]
    pub fn from_pad_axes(event: E, timeout: Timeout, axes: &[GamepadAxisType]) -> KeySequence<E> {
        Self {
            event,
            timeout,
            inputs: VecDeque::from_iter(axes.iter().copied().map(Entry::PadAxis)),
        }
    }


    #[inline(always)]
    pub fn new(event: E, inputs: &[Entry], timeout: Timeout) -> KeySequence<E> {
        Self {
            event,
            timeout,
            inputs: VecDeque::from_iter(inputs.iter().cloned()),
        }
    }


    #[inline(always)]
    pub fn is_last(&self) -> bool {
        self.inputs.len() <= 1
    }


    #[inline(always)]
    pub fn next_input(&self) -> Option<Entry> {
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