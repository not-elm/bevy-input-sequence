use bevy::prelude::{Component, Event, GamepadAxisType, GamepadButtonType, KeyCode};

use crate::input::Entry;
use crate::prelude::Timeout;
use crate::sequence::KeySequence;

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
    pub fn from_pad_button_axes(
        event: E,
        timeout: Timeout,
        buttons: &[GamepadButtonType],
    ) -> InputSequence<E> {
        Self(KeySequence::from_pad_button_axes(event, timeout, buttons))
    }


    #[inline(always)]
    pub fn from_pad_axes(
        event: E,
        timeout: Timeout,
        axes: &[GamepadAxisType],
    ) -> InputSequence<E> {
        Self(KeySequence::from_pad_axes(event, timeout, axes))
    }



    #[inline(always)]
    pub fn new(
        event: E,
        timeout: Timeout,
        inputs: &[Entry],
    ) -> InputSequence<E> {
        Self(KeySequence::new(event, inputs, timeout))
    }


    #[inline(always)]
    pub(crate) fn event(&self) -> E {
        self.0.event()
    }


    #[inline(always)]
    pub(crate) fn next_input(&self) -> Option<Entry> {
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


pub struct Builder<E>{
    event: E,
    timeout: Timeout,
    inputs: Vec<Entry>
}


impl<E: Event + Clone> Builder<E> {
    fn new(event: E) -> Builder<E>{
        Self{
            event,
            timeout: Timeout::default(),
            inputs: vec![]
        }
    }


    fn timeout(mut self, timeout: Timeout) -> Builder<E>{
        self.timeout = timeout;
        self
    }

}
