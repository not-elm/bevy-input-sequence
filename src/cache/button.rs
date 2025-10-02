//! Cache the trie for reuse.
use crate::input_sequence::InputSequence;
use bevy::prelude::{Entity, GamepadButton, In, Resource};
use std::collections::HashMap;
use trie_rs::{
    inc_search::{IncSearch, Position},
    map::{Trie, TrieBuilder},
};

/// Contains the trie for gamepad button sequences.
#[derive(Resource, Default)]
pub struct ButtonSequenceCache {
    trie: Option<Trie<GamepadButton, InputSequence<GamepadButton, In<Entity>>>>,
    position: HashMap<Entity, Position>,
}

impl ButtonSequenceCache {
    /// Retrieve the cached trie without iterating through `sequences`. Or if
    /// the cache has been invalidated, build and cache a new trie using the
    /// `sequences` iterator.
    pub fn trie<'a>(
        &mut self,
        sequences: impl Iterator<Item = &'a InputSequence<GamepadButton, In<Entity>>>,
    ) -> &Trie<GamepadButton, InputSequence<GamepadButton, In<Entity>>> {
        self.trie.get_or_insert_with(|| {
            let mut builder: TrieBuilder<GamepadButton, InputSequence<GamepadButton, In<Entity>>> =
                TrieBuilder::new();
            for sequence in sequences {
                builder.insert(sequence.acts.clone(), sequence.clone());
            }
            // info!(
            //     "Building trie for {} input sequences.",
            //     A::short_type_path()
            // );
            assert!(
                self.position.is_empty(),
                "Position should be none when rebuilding trie"
            );
            builder.build()
        })
    }

    /// Store a search.
    pub fn store(&mut self, key: Entity, position: Position) {
        self.position.insert(key, position);
    }

    /// Recall a search OR create a new search.
    pub fn recall<'a, 'b>(
        &'b mut self,
        key: Entity,
        sequences: impl Iterator<Item = &'a InputSequence<GamepadButton, In<Entity>>>,
    ) -> IncSearch<'a, GamepadButton, InputSequence<GamepadButton, In<Entity>>>
    where
        'b: 'a,
    {
        let position = self.position.get(&key).cloned();
        let trie = self.trie(sequences);
        position
            .map(move |p| IncSearch::resume(trie, p))
            .unwrap_or_else(move || trie.inc_search())
    }

    /// Clears the cache.
    pub fn reset(&mut self) {
        self.trie = None;
        self.position.clear();
    }
}
