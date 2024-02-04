use bevy::prelude::{Component, Event};
use bevy::time::Time;

use crate::act::Act;
use crate::prelude::Timeout;
use crate::SequenceReader;

#[derive(Component, Debug, Clone)]
pub struct InputSequence<E> {
    pub(crate) event: E,
    timeout: Timeout,
    pub(crate) inputs: Vec<Act>,
}

impl<E> InputSequence<E>
where
    E: Event + Clone,
{
    #[inline(always)]
    pub fn new<T>(
        event: E,
        inputs: impl IntoIterator<Item = T>,
    ) -> InputSequence<E>
    where
        T: Into<Act>,
    {
        let r = Self {
            event,
            timeout: Timeout::None,
            inputs: Vec::from_iter(inputs.into_iter().map(|x| x.into())),
        };
        assert!(
            r.inputs.len() > 0,
            "input sequence must have one or more inputs."
        );
        r
    }

    pub fn timeout(mut self, timeout: impl Into<Timeout>) -> Self {
        self.timeout = timeout.into();
        self
    }

    #[inline(always)]
    pub(crate) fn one_key(&self) -> bool {
        1 == self.inputs.len()
    }

    pub fn first_input(&self) -> &Act {
        &self.inputs[0]
    }

    #[inline(always)]
    pub fn event(&self) -> &E {
        &self.event
    }

    #[inline(always)]
    pub(crate) fn timedout(&mut self, time: &Time) -> bool {
        self.timeout.timedout(time)
    }

    #[inline(always)]
    pub(crate) fn start_reader(self, at: usize) -> SequenceReader<E> {
        SequenceReader::new(self, at)
    }
}
