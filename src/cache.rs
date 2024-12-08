//! Cache the trie for reuse.
use crate::input_sequence::InputSequence;
use bevy::{ecs::system::Resource, reflect::TypePath, prelude::{Reflect, ReflectResource}};
use std::{collections::HashMap, hash::Hash};
use trie_rs::{
    inc_search::{IncSearch, Position},
    map::{Trie, TrieBuilder},
};

/// Contains the trie for the input sequences.
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct InputSequenceCache<A, In> {
    trie: Option<Trie<A, InputSequence<A, In>>>,
    position: HashMap<In, Position>,
}

impl<A, In> InputSequenceCache<A, In>
where
    A: Ord + Clone + Send + Sync + TypePath + 'static,
    In: Send + Sync + Clone + Eq + Hash + 'static,
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
    pub fn store(&mut self, key: In, position: Position) {
        self.position.insert(key, position);
    }

    /// Recall a search OR create a new search.
    pub fn recall<'a, 'b>(
        &'b mut self,
        key: In,
        sequences: impl Iterator<Item = &'a InputSequence<A, In>>,
    ) -> IncSearch<'a, A, InputSequence<A, In>>
    where
        'b: 'a,
    {
        let position = self.position.get(&key).cloned();
        let trie = self.trie(sequences);
        position
            .map(move |p| IncSearch::resume(trie, p))
            .unwrap_or_else(move || trie.inc_search())
    }
}

impl<A, In> InputSequenceCache<A, In> {
    /// Clears the cache.
    pub fn reset(&mut self) {
        self.trie = None;
        self.position.clear();
    }
}

impl<A, In> Default for InputSequenceCache<A, In> {
    fn default() -> Self {
        Self {
            trie: None,
            position: HashMap::new(),
        }
    }
}
