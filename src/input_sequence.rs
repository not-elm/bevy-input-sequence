use bevy::prelude::{Component, Event, GamepadButtonType, KeyCode};

use crate::act::Act;
use crate::key_sequence::KeySequence;
use crate::prelude::Timeout;

#[derive(Component)]
pub struct InputSequence<E>(KeySequence<E>);


impl<E: Event + Clone> InputSequence<E> {
    #[inline(always)]
    pub fn from_keycodes(
        event: E,
        timeout: Timeout,
        keycodes: &[KeyCode],
    ) -> InputSequence<E> {
        Self(KeySequence::from_keycodes(event, timeout, keycodes))
    }


    #[inline(always)]
    pub fn from_pad_buttons(
        event: E,
        timeout: Timeout,
        buttons: &[GamepadButtonType],
    ) -> InputSequence<E> {
        Self(KeySequence::from_pad_buttons(event, timeout, buttons))
    }


    #[inline(always)]
    pub fn new(
        event: E,
        timeout: Timeout,
        inputs: &[Act],
    ) -> InputSequence<E> {
        Self(KeySequence::new(event, inputs, timeout))
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


