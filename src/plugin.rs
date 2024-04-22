use bevy::{
    app::{App, Plugin, Update},
    core::FrameCount,
    ecs::{
        query::Added,
        removal_detection::RemovedComponents,
        schedule::{IntoSystemConfigs, ScheduleLabel, SystemSet},
        system::{Commands, Local, Query, Res, ResMut},
    },
    input::{
        gamepad::{Gamepad, GamepadButton, GamepadButtonType},
        keyboard::KeyCode,
        ButtonInput,
    },
    log::warn,
    time::Time,
    utils::intern::Interned,
};
use std::collections::HashMap;

use crate::{
    cache::InputSequenceCache,
    chord::is_modifier,
    covec::Covec,
    frame_time::FrameTime,
    input_sequence::{ButtonSequence, InputSequence, KeySequence},
    KeyChord, Modifiers,
};
use trie_rs::{
    map::Trie,
    inc_search::{IncSearch, Answer},
};

/// ButtonInput sequence plugin.
pub struct InputSequencePlugin {
    #[allow(clippy::type_complexity)]
    schedules: Vec<(Interned<dyn ScheduleLabel>, Option<Interned<dyn SystemSet>>)>,
    match_key: Option<bool>,
    match_button: Option<bool>,
}

impl Default for InputSequencePlugin {
    fn default() -> Self {
        InputSequencePlugin {
            schedules: vec![(Interned(Box::leak(Box::new(Update))), None)],
            match_key: None,
            match_button: None,
        }
    }
}

impl Plugin for InputSequencePlugin {
    fn build(&self, app: &mut App) {
        if self
            .match_key
            .unwrap_or(app.world.get_resource::<ButtonInput<KeyCode>>().is_some())
        {
            // Add key sequence.
            app.init_resource::<InputSequenceCache<KeyChord, ()>>();

            for (schedule, set) in &self.schedules {
                if let Some(set) = set {
                    app.add_systems(
                        *schedule,
                        (
                            detect_removals::<KeyChord, ()>,
                            detect_additions::<KeyChord, ()>,
                            key_sequence_matcher,
                        )
                            .chain()
                            .in_set(*set),
                    );
                } else {
                    app.add_systems(
                        *schedule,
                        (
                            detect_removals::<KeyChord, ()>,
                            detect_additions::<KeyChord, ()>,
                            key_sequence_matcher,
                        )
                            .chain(),
                    );
                }
            }
        } else {
            warn!("No key sequence matcher added; consider adding DefaultPlugins.");
        }

        if self.match_button.unwrap_or(
            app.world
                .get_resource::<ButtonInput<GamepadButton>>()
                .is_some(),
        ) {
            // Add button sequences.
            app.init_resource::<InputSequenceCache<GamepadButtonType, Gamepad>>();

            for (schedule, set) in &self.schedules {
                if let Some(set) = set {
                    app.add_systems(
                        *schedule,
                        (
                            detect_removals::<GamepadButtonType, Gamepad>,
                            detect_additions::<GamepadButtonType, Gamepad>,
                            button_sequence_matcher,
                        )
                            .chain()
                            .in_set(*set),
                    );
                } else {
                    app.add_systems(
                        *schedule,
                        (
                            detect_removals::<GamepadButtonType, Gamepad>,
                            detect_additions::<GamepadButtonType, Gamepad>,
                            button_sequence_matcher,
                        )
                            .chain(),
                    );
                }
            }
        } else {
            warn!("No button sequence matcher added; consider adding DefaultPlugins.");
        }
    }
}

impl InputSequencePlugin {
    /// Constructs an empty input sequence plugin with no default schedules.
    pub fn empty() -> Self {
        Self {
            schedules: vec![],
            match_key: None,
            match_button: None,
        }
    }
    /// Run the executor in a specific `Schedule`.
    pub fn run_in(mut self, schedule: impl ScheduleLabel) -> Self {
        self.schedules
            .push((Interned(Box::leak(Box::new(schedule))), None));
        self
    }

    /// Run the executor in a specific `Schedule` and `SystemSet`.
    pub fn run_in_set(mut self, schedule: impl ScheduleLabel, set: impl SystemSet) -> Self {
        self.schedules.push((
            Interned(Box::leak(Box::new(schedule))),
            Some(Interned(Box::leak(Box::new(set)))),
        ));
        self
    }

    /// Run systems to match keys. By default will match keys if resource
    /// `ButtonInput<KeyCode>` exists.
    pub fn match_key(mut self, yes: bool) -> Self {
        self.match_key = Some(yes);
        self
    }

    /// Run systems to match button. By default will match keys if resource
    /// `ButtonInput<GamepadButton>` exists.
    pub fn match_button(mut self, yes: bool) -> Self {
        self.match_button = Some(yes);
        self
    }
}

fn detect_additions<A: Clone + Send + Sync + 'static, In: 'static>(
    secrets: Query<&InputSequence<A, In>, Added<InputSequence<A, In>>>,
    mut cache: ResMut<InputSequenceCache<A, In>>,
) {
    if secrets.iter().next().is_some() {
        cache.reset();
    }
}

fn detect_removals<A: Clone + Send + Sync + 'static, In: 'static>(
    mut cache: ResMut<InputSequenceCache<A, In>>,
    mut removals: RemovedComponents<InputSequence<A, In>>,
) {
    if removals.read().next().is_some() {
        cache.reset();
    }
}

#[allow(clippy::too_many_arguments)]
fn button_sequence_matcher(
    secrets: Query<&ButtonSequence>,
    time: Res<Time>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut last_buttons: Local<HashMap<usize, Covec<GamepadButtonType, FrameTime>>>,
    mut cache: ResMut<InputSequenceCache<GamepadButtonType, Gamepad>>,
    frame_count: Res<FrameCount>,
    mut commands: Commands,
) {
    let trie = cache.trie(secrets.iter());
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_seconds(),
    };
    for button in buttons.get_just_pressed() {
        let pad_buttons = match last_buttons.get_mut(&button.gamepad.id) {
            Some(x) => x,
            None => {
                last_buttons.insert(button.gamepad.id, Covec::default());
                last_buttons.get_mut(&button.gamepad.id).unwrap()
            }
        };

        pad_buttons.push(button.button_type, now.clone());
        let start = pad_buttons.1[0].clone();
        for seq in consume_input(trie, &mut pad_buttons.0) {
            if seq
                .time_limit
                .as_ref()
                .map(|limit| (&now - &start).has_timedout(limit))
                .unwrap_or(false)
            {
                // Sequence timed out.
            } else {
                commands.run_system_with_input(seq.system_id, button.gamepad);
            }
        }
        pad_buttons.drain1_sync();
    }
}

#[allow(clippy::too_many_arguments)]
fn key_sequence_matcher(
    secrets: Query<&KeySequence>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut last_keys: Local<Covec<KeyChord, FrameTime>>,
    mut cache: ResMut<InputSequenceCache<KeyChord, ()>>,
    frame_count: Res<FrameCount>,
    mut commands: Commands,
) {
    let mods = Modifiers::from_input(&keys);
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_seconds(),
    };
    let maybe_start = last_keys.1.first().cloned();
    let mut input = keys.get_just_pressed()
        .filter(|k| ! is_modifier(**k))
        .map(|k| {
            let chord = KeyChord(mods, *k);
            last_keys.push(chord.clone(), now.clone());
            chord
        })
        .peekable();
    if input.peek().is_none() {
        return;
    }
    let mut search = cache.recall(secrets.iter());
    let mut min_prefix = None;

    // eprintln!("maybe_start {maybe_start:?} now {now:?}");
    for seq in inc_consume_input(&mut search,
                                 input,
                                 &mut min_prefix) {
        if let Some(ref start) = maybe_start {
            if seq
            .time_limit
            .as_ref()
            .map(|limit| (&now - &start).has_timedout(limit))
            .unwrap_or(false)
            {
                // Sequence timed out.
                continue;
            }
        }
        commands.run_system(seq.system_id);
    }
    // eprintln!("min_prefix {min_prefix:?}");
    match min_prefix {
        Some(i) => {
            let _ = last_keys.0.drain(0..i);
        }
        None => {
            last_keys.0.clear();
        }
    }
    last_keys.drain1_sync();
    let position = search.into();
    cache.store(position);
}

/// Incrementally consume the input.
fn inc_consume_input<'a, 'b, K, V>(search: &'b mut IncSearch<'a, K, V>,
                                   input: impl Iterator<Item = K> + 'b,
                                   min_prefix: &'b mut Option<usize>)
                                   -> impl Iterator<Item = &'a V> + 'b
where
    K: Clone + Eq + Ord,
    'a: 'b,
{
    let mut i = 0;
    input.filter_map(move |k| {
        i += 1;
        match search.query(&k) {
            Some(Answer::Match) => {
                let result = Some(search.value().unwrap());
                search.reset();
                *min_prefix = None;
                result
            }
            Some(Answer::Prefix) if min_prefix.is_none() => {
                *min_prefix = Some(i - 1);
                None
            }
            Some(Answer::PrefixAndMatch) => {
                Some(search.value().unwrap())
            }
            Some(Answer::Prefix) => {
                None
            }
            None => {
                search.reset();
                *min_prefix = None;
                // This could be the start of a new sequence.
                if search.query(&k).is_none() {
                    // This may not be necessary.
                    search.reset();
                } else {
                    *min_prefix = Some(i - 1);
                }
                None
            }
        }
    })
}

fn consume_input<'a, K, V>(trie: &'a Trie<K, V>, input: &mut Vec<K>) -> impl Iterator<Item = &'a V>
where
    K: Clone + Eq + Ord,
{
    let mut result = vec![];
    let mut min_prefix = None;
    for i in 0..input.len() {
        if let Some(seq) = trie.exact_match(&input[i..]) {
            result.push(seq);
        }
        if min_prefix.is_none() && trie.is_prefix(&input[i..]) {
            min_prefix = Some(i);
            // let _ = input.drain(0..i);
            // return result.into_iter();
        }
    }
    match min_prefix {
        Some(i) => {
            let _ = input.drain(0..i);
        }
        None => {
            input.clear();
        }
    }
    result.into_iter()
}
