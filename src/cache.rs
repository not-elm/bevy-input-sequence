use bevy::{
    app::{App, Plugin, Update},
    core::FrameCount,
    ecs::{
        schedule::{ScheduleLabel, SystemSet},
        system::Commands,
    },
    log::warn,
    prelude::{
        Added, ButtonInput as Input, Gamepad, GamepadButton, GamepadButtonType, IntoSystemConfigs,
        KeyCode, Local, Query, RemovedComponents, Res, ResMut, Resource,
    },
    reflect::{Enum, Reflect},
    time::Time,
    utils::intern::Interned,
};
use std::collections::HashMap;
use std::fmt;
use trie_rs::map::{Trie, TrieBuilder};
use super::InputSequence;

/// Contains the trie for the input sequences.
#[derive(Resource)]
pub struct InputSequenceCache<A, In> {
    pub(crate) trie: Option<Trie<A, InputSequence<A, In>>>,
}

impl<A, In> InputSequenceCache<A, In>
where
    A: Ord + Clone + Send + Sync + 'static,
    In: Send + Sync + Clone + 'static,
{
    /// Retrieve the cached trie without iterating through `sequences`. Or if
    /// the cache has been invalidated, build and cache a new trie using the
    /// `sequences` iterator.
    pub fn trie<'a>(
        &mut self,
        sequences: impl Iterator<Item = &'a InputSequence<A, In>>,
    ) -> &Trie<A, InputSequence<A, In>> {
        self.trie.get_or_insert_with(|| {
            let mut builder: TrieBuilder<A, InputSequence<A, In>> = TrieBuilder::new();
            for sequence in sequences {
                builder.insert(sequence.acts.clone(), sequence.clone());
            }
            builder.build()
        })
    }
}

impl<A, In> Default for InputSequenceCache<A, In> {
    fn default() -> Self {
        Self { trie: None }
    }
}
