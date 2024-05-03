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
use std::collections::{HashMap, VecDeque};

use crate::{
    cache::InputSequenceCache,
    chord::is_modifier,
    frame_time::FrameTime,
    input_sequence::{ButtonSequence, InputSequence, KeySequence},
    KeyChord, Modifiers,
};
use trie_rs::inc_search::{Answer, IncSearch};

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

fn detect_additions<A: Clone + Send + Sync + 'static, In: Send + Sync + 'static>(
    sequences: Query<&InputSequence<A, In>, Added<InputSequence<A, In>>>,
    mut cache: ResMut<InputSequenceCache<A, In>>,
) {
    if sequences.iter().next().is_some() {
        cache.reset();
    }
}

fn detect_removals<A: Clone + Send + Sync + 'static, In: Send + Sync + 'static>(
    mut cache: ResMut<InputSequenceCache<A, In>>,
    mut removals: RemovedComponents<InputSequence<A, In>>,
) {
    if removals.read().next().is_some() {
        cache.reset();
    }
}

#[allow(clippy::too_many_arguments)]
fn button_sequence_matcher(
    sequences: Query<&ButtonSequence>,
    time: Res<Time>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut last_times: Local<HashMap<usize, VecDeque<FrameTime>>>,
    mut cache: ResMut<InputSequenceCache<GamepadButtonType, Gamepad>>,
    frame_count: Res<FrameCount>,
    mut commands: Commands,
) {
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_seconds(),
    };
    for button in buttons.get_just_pressed() {
        let last_times = match last_times.get_mut(&button.gamepad.id) {
            Some(x) => x,
            None => {
                last_times.insert(button.gamepad.id, VecDeque::new());
                last_times.get_mut(&button.gamepad.id).unwrap()
            }
        };

        last_times.push_back(now.clone());
        let start = &last_times[0];
        let mut search = cache.recall(button.gamepad, sequences.iter().by_ref());
        for seq in inc_consume_input(&mut search, std::iter::once(button.button_type)) {
            if seq
                .time_limit
                .as_ref()
                .map(|limit| (&now - start).has_timedout(limit))
                .unwrap_or(false)
            {
                // Sequence timed out.
            } else {
                commands.run_system_with_input(seq.system_id, button.gamepad);
            }
        }
        let prefix_len = search.prefix_len();
        let l = last_times.len();
        let _ = last_times.drain(0..l - prefix_len);
        let position = search.into();
        cache.store(button.gamepad, position);
    }
}

#[allow(clippy::too_many_arguments)]
fn key_sequence_matcher(
    sequences: Query<&KeySequence>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut last_times: Local<VecDeque<FrameTime>>,
    mut cache: ResMut<InputSequenceCache<KeyChord, ()>>,
    frame_count: Res<FrameCount>,
    mut commands: Commands,
) {
    let mods = Modifiers::from_input(&keys);
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_seconds(),
    };
    let maybe_start = last_times.front().cloned();
    let mut input = keys
        .get_just_pressed()
        .filter(|k| !is_modifier(**k))
        .map(|k| {
            let chord = KeyChord(mods, *k);
            last_times.push_back(now.clone());
            chord
        })
        .peekable();
    if input.peek().is_none() {
        return;
    }
    let mut search = cache.recall((), sequences.iter());

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
    cache.store((), position);
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
                if search.query(&k).is_none() {
                    // This may not be necessary.
                    search.reset();
                }
                None
            }
        }
    })
}
