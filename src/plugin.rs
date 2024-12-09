use bevy::{
    app::{App, Plugin, Update},
    core::FrameCount,
    ecs::{
        entity::Entity,
        prelude::In,
        intern::Interned,
        query::Added,
        removal_detection::RemovedComponents,
        schedule::{IntoSystemConfigs, ScheduleLabel, SystemSet},
        system::{Commands, Local, Query, Res, ResMut, SystemInput},
    },
    input::{
        gamepad::{Gamepad, GamepadButton},
        keyboard::KeyCode,
        ButtonInput,
    },
    log::warn,
    time::Time,
};
use std::collections::{HashMap, VecDeque};

use crate::{
    cache::{ButtonSequenceCache, KeySequenceCache},
    chord::{is_modifier, KeyChordQueue},
    frame_time::FrameTime,
    input_sequence::{ButtonSequence, InputSequence, KeySequence},
    KeyChord, Modifiers,
};
use trie_rs::inc_search::{Answer, IncSearch, Position};

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
            .unwrap_or(app.world().get_resource::<ButtonInput<KeyCode>>().is_some())
        {
            app
                .register_type::<InputSequence<KeyChord, ()>>()
                // .register_type::<InputSequenceCache<KeyChord, ()>>()
                ;
            // Add key sequence.
            app.init_resource::<KeySequenceCache>();
            app.init_resource::<KeyChordQueue>();

            for (schedule, set) in &self.schedules {
                if let Some(set) = set {
                    app.add_systems(
                        *schedule,
                        (
                            detect_key_removals,
                            detect_key_additions,
                            key_sequence_matcher,
                        )
                            .chain()
                            .in_set(*set),
                    );
                } else {
                    app.add_systems(
                        *schedule,
                        (
                            detect_key_removals,
                            detect_key_additions,
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
            false
            // NOTE: Is there a way to detect whether gamepad input is available post 0.14?
            // app.world()
            //     .get_resource::<ButtonInput<GamepadButton>>()
            //     .is_some(),
        ) {
            // app
            //     .register_type::<InputSequence<GamepadButton, In<Entity>>>()
            //     // .register_type::<InputSequenceCache<GamepadButton, Gamepad>>()
            //     ;
            // Add button sequences.
            app.init_resource::<ButtonSequenceCache>();

            for (schedule, set) in &self.schedules {
                if let Some(set) = set {
                    app.add_systems(
                        *schedule,
                        (
                            detect_button_removals,
                            detect_button_additions,
                            button_sequence_matcher,
                        )
                            .chain()
                            .in_set(*set),
                    );
                } else {
                    app.add_systems(
                        *schedule,
                        (
                            detect_button_removals,
                            detect_button_additions,
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

fn detect_key_additions(
    sequences: Query<&InputSequence<KeyChord, ()>, Added<InputSequence<KeyChord, ()>>>,
    mut cache: ResMut<KeySequenceCache>,
)
{
    if sequences.iter().next().is_some() {
        cache.reset();
    }
}

fn detect_button_additions(
    sequences: Query<&InputSequence<GamepadButton, In<Entity>>, Added<InputSequence<GamepadButton, In<Entity>>>>,
    mut cache: ResMut<ButtonSequenceCache>,
)
{
    if sequences.iter().next().is_some() {
        cache.reset();
    }
}

fn detect_key_removals(
    mut cache: ResMut<KeySequenceCache>,
    mut removals: RemovedComponents<InputSequence<KeyChord, ()>>,
) {
    if removals.read().next().is_some() {
        cache.reset();
    }
}

fn detect_button_removals(
    mut cache: ResMut<ButtonSequenceCache>,
    mut removals: RemovedComponents<InputSequence<GamepadButton, In<Entity>>>,
) {
    if removals.read().next().is_some() {
        cache.reset();
    }
}

#[allow(clippy::too_many_arguments)]
fn button_sequence_matcher(
    sequences: Query<&ButtonSequence>,
    time: Res<Time>,
    mut last_times: Local<HashMap<Entity, VecDeque<FrameTime>>>,
    mut cache: ResMut<ButtonSequenceCache>,
    frame_count: Res<FrameCount>,
    mut commands: Commands,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_secs(),
    };
    for (id, gamepad) in &gamepads {
        for button in gamepad.get_just_pressed() {
            let last_times = match last_times.get_mut(&id) {
                Some(x) => x,
                None => {
                    last_times.insert(id, VecDeque::new());
                    last_times.get_mut(&id).unwrap()
                }
            };

            last_times.push_back(now.clone());
            let start = &last_times[0];
            let mut search = cache.recall(id, sequences.iter().by_ref());
            for seq in inc_consume_input(&mut search, std::iter::once(*button)) {
                if seq
                    .time_limit
                    .as_ref()
                    .map(|limit| (&now - start).has_timedout(limit))
                    .unwrap_or(false)
                {
                    // Sequence timed out.
                } else {
                    commands.run_system_with_input(seq.system_id, id);
                }
            }
            let prefix_len = search.prefix_len();
            let l = last_times.len();
            let _ = last_times.drain(0..l - prefix_len);
            let position = search.into();
            cache.store(id, position);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn key_sequence_matcher(
    sequences: Query<&KeySequence>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut last_times: Local<VecDeque<FrameTime>>,
    mut cache: ResMut<KeySequenceCache>,
    frame_count: Res<FrameCount>,
    mut commands: Commands,
    mut keychord_queue: ResMut<KeyChordQueue>,
) {
    let mods = Modifiers::from(&keys);
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_secs(),
    };
    let maybe_start = last_times.front().cloned();
    let mut input = keychord_queue
        .drain(..)
        .chain(
            keys.get_just_pressed()
                .filter(|k| !is_modifier(**k))
                .map(|k| {
                    let chord = KeyChord(mods, *k);
                    last_times.push_back(now.clone());
                    chord
                }),
        )
        .peekable();
    if input.peek().is_none() {
        return;
    }

    let mut search = cache.recall(sequences.iter());

    // eprintln!("maybe_start {maybe_start:?} now {now:?}");
    for seq in inc_consume_input(&mut search, input) {
        if let Some(ref start) = maybe_start {
            if seq
                .time_limit
                .as_ref()
                .map(|limit| (&now - start).has_timedout(limit))
                .unwrap_or(false)
            {
                // Sequence timed out.
                continue;
            }
        }
        commands.run_system(seq.system_id);
    }
    let prefix_len = search.prefix_len();
    let l = last_times.len();
    let _ = last_times.drain(0..l - prefix_len);
    let position = search.into();
    cache.store(position);
}

/// Incrementally consume the input.
fn inc_consume_input<'a, 'b, K, V>(
    search: &'b mut IncSearch<'a, K, V>,
    input: impl Iterator<Item = K> + 'b,
) -> impl Iterator<Item = &'a V> + 'b
where
    K: Clone + Eq + Ord,
    'a: 'b,
{
    input.filter_map(move |k| {
        match search.query(&k) {
            Some(Answer::Match) => {
                let result = Some(search.value().unwrap());
                search.reset();
                result
            }
            Some(Answer::PrefixAndMatch) => Some(search.value().unwrap()),
            Some(Answer::Prefix) => None,
            None => {
                search.reset();
                // This could be the start of a new sequence.
                //
                // Let's check it.
                match search.query(&k) {
                    Some(Answer::Match) => {
                        let result = Some(search.value().unwrap());
                        search.reset();
                        result
                    }
                    Some(Answer::PrefixAndMatch) => Some(search.value().unwrap()),
                    Some(Answer::Prefix) => None,
                    None => {
                        // This may not be necessary.
                        search.reset();
                        None
                    }
                }
            }
        }
    })
}
