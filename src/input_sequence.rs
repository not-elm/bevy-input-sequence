use std::ops::{Deref,DerefMut};
use bevy::prelude::{Component, Event, GamepadButtonType};

use crate::GamepadEvent;
// use crate::act::Acts;
use crate::act::KeyChord;
use crate::time_limit::TimeLimit;

/// An input sequence is a series of [ActPattern]s that fires an event when
/// matched with inputs within the given time limit.
#[derive(Component, Debug, Clone)]
pub struct InputSequence<E,A> {
    /// Event emitted
    pub event: E,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<A>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
}

impl<E, A> InputSequence<E,A>
where
    E: Event + Clone,
    A: Clone
{
    /// Create new input sequence. Not operant until added to an entity.
    #[inline(always)]
    pub fn new<T>(event: E, acts: impl IntoIterator<Item = T>)
                        -> InputSequence<E,A>
    where
        A: From<T>,
    {
        Self {
            event,
            time_limit: None,
            acts: Vec::from_iter(acts.into_iter().map(A::from)),
        }
    }

    /// Specify a time limit from the start of the first matching input.
    pub fn time_limit(mut self, time_limit: impl Into<TimeLimit>) -> Self {
        self.time_limit = Some(time_limit.into());
        self
    }
}

/// Represents a key sequence.
pub type KeySequence<E> = InputSequence<E, KeyChord>;
// pub type ButtonSequence<E> = InputSequence<E, GamepadButtonType>;
/// Represents a game pad button sequence.
#[derive(Component, Debug, Clone)]
pub struct ButtonSequence<E>(pub(crate) InputSequence<E, GamepadButtonType>);

// impl<E> Deref for ButtonSequence<E> {
//     type Target = InputSequence<E, GamepadButtonType>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<E> DerefMut for ButtonSequence<E> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }


impl<E> ButtonSequence<E>
where
    E: GamepadEvent + Clone,
{
    /// Create new input sequence. Not operant until added to an entity.
    #[inline(always)]
    pub fn new(event: E, acts: impl IntoIterator<Item = GamepadButtonType>)
                        -> ButtonSequence<E>
    {
        ButtonSequence(InputSequence {
            event,
            time_limit: None,
            acts: Vec::from_iter(acts.into_iter())
        })
    }
    /// Specify a time limit from the start of the first matching input.
    pub fn time_limit(mut self, time_limit: impl Into<TimeLimit>) -> Self {
        self.0.time_limit = Some(time_limit.into());
        self
    }

}
