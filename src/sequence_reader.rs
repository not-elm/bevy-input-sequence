use bevy::prelude::{Component, Event, Time};

use crate::act::Act;
use crate::input_sequence::InputSequence;
use crate::timeout::Timeout;
use crate::InputParams;

/// Reads input matching or not against a given input sequence.
#[derive(Component)]
pub(crate) struct SequenceReader<E> {
    seq: Option<InputSequence<E>>,
    index: usize,
    timeout: Timeout,
    pub(crate) context: Option<usize>, // buttons_filter: Option<F>,
}

impl<E: Event + Clone> SequenceReader<E> {
    #[inline(always)]
    pub(crate) fn new(seq: InputSequence<E>, start_index: usize, context: Option<usize>) -> SequenceReader<E> {
        let timeout = seq.time_limit.clone().into();
        Self {
            seq: Some(seq),
            index: start_index,
            timeout: timeout,
            context,
        }
    }

    /// Returns the event. Repeated calls to `event()` will panic.
    #[inline(always)]
    pub(crate) fn event(&mut self) -> E {
        self.seq.take().expect("No input sequence in reader").event
    }

    #[inline(always)]
    pub(crate) fn next_input(&self) -> Option<&Act> {
        self.seq.as_ref().and_then(|x| x.acts.get(self.index))
    }

    #[inline(always)]
    pub(crate) fn next_act(&mut self) {
        self.index += 1;
    }

    #[inline(always)]
    pub(crate) fn is_last(&self) -> bool {
        self.seq
            .as_ref()
            .map(|x| self.index >= x.acts.len())
            .unwrap_or(true)
    }

    #[inline(always)]
    pub(crate) fn timedout(&mut self, time: &Time) -> bool {
        self.timeout.timedout(time)
    }

    pub(crate) fn just_other_inputted(&self, inputs: &InputParams, next_input: &Act) -> bool {
        next_input.other_pressed_keycode(inputs.key.get_just_pressed())
            || next_input.other_pressed_pad_button(
                inputs
                    .button_inputs
                    .get_just_pressed()
                    // Only account for gamepads that start the sequence.
                    .filter(|button| self.context.map_or(true, |x| x == button.gamepad.id)),
            )
    }
}
