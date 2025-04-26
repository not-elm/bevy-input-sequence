//! Cache the trie for reuse.
use crate::{input_sequence::InputSequence, KeyChord};
use bevy::ecs::prelude::Resource;
use trie_rs::{
    inc_search::{IncSearch, Position},
    map::{Trie, TrieBuilder},
};

/// Contains the trie for gamepad button sequences.
#[derive(Resource, Default)]
pub struct KeySequenceCache {
    trie: Option<Trie<KeyChord, InputSequence<KeyChord, ()>>>,
    position: Option<Position>,
}

impl KeySequenceCache {
    /// Retrieve the cached trie without iterating through `sequences`. Or if
    /// the cache has been invalidated, build and cache a new trie using the
    /// `sequences` iterator.
    pub fn trie<'a>(
        &mut self,
        sequences: impl Iterator<Item = &'a InputSequence<KeyChord, ()>>,
    ) -> &Trie<KeyChord, InputSequence<KeyChord, ()>> {
        self.trie.get_or_insert_with(|| {
            let mut builder: TrieBuilder<KeyChord, InputSequence<KeyChord, ()>> =
                TrieBuilder::new();
            for sequence in sequences {
                builder.insert(sequence.acts.clone(), sequence.clone());
            }
            // info!(
            //     "Building trie for {} input sequences.",
            //     A::short_type_path()
            // );
            builder.build()
        })
    }

    /// Store a search.
    pub fn store(&mut self, position: Position) {
        self.position = Some(position)
    }

    /// Recall a search OR create a new search.
    pub fn recall<'a, 'b>(
        &'b mut self,
        sequences: impl Iterator<Item = &'a InputSequence<KeyChord, ()>>,
    ) -> IncSearch<'a, KeyChord, InputSequence<KeyChord, ()>>
    where
        'b: 'a,
    {
        let position = self.position;
        let trie = self.trie(sequences);
        position
            .map(move |p| IncSearch::resume(trie, p))
            .unwrap_or_else(move || trie.inc_search())
    }

    /// Clears the cache.
    pub fn reset(&mut self) {
        self.trie = None;
        self.position = None;
    }
}
