#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.3.0")]
#![doc = include_str!("../README.md")]
// #![forbid(missing_docs)]
use bevy::{
    log::warn,
    app::{App, Plugin, Update},
    core::FrameCount,
    ecs::{
        schedule::{ScheduleLabel, SystemSet},
        system::Commands,
    },
    prelude::{
        Added, ButtonInput as Input, Gamepad, GamepadButton, GamepadButtonType,
        IntoSystemConfigs, KeyCode, Local, Query, RemovedComponents, Res,
        ResMut, Resource,
    },
    reflect::{Enum, Reflect},
    time::Time,
    utils::intern::Interned,
};
use std::collections::HashMap;
use std::fmt;
use trie_rs::map::{Trie, TrieBuilder};

pub use crate::input_sequence::{ButtonSequence, InputSequence, KeySequence, IntoCondSystem, Blanket};
pub use crate::time_limit::TimeLimit;

pub use keyseq::{
    bevy::{pkey as key, pkeyseq as keyseq},
    Modifiers,
};

pub mod action;
mod covec;
mod frame_time;
mod input_sequence;
mod time_limit;

use covec::Covec;
use frame_time::FrameTime;

/// Represents a key chord, i.e., a set of modifiers and a key code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct KeyChord(pub Modifiers, pub KeyCode);

impl fmt::Display for KeyChord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use KeyCode::*;
        self.0.fmt(f)?;
        if !self.0.is_empty() {
            f.write_str("-")?;
        }
        let key_repr = match self.1 {
            Semicolon => ";",
            Period => ".",
            Equal => "=",
            Slash => "/",
            Minus => "-",
            BracketLeft => "[",
            BracketRight => "]",
            Quote => "'",
            Backquote => "`",
            key_code => {
                let mut key = key_code.variant_name();
                key = key.strip_prefix("Key").unwrap_or(key);
                key = key.strip_prefix("Digit").unwrap_or(key);
                return f.write_str(key);
            }
        };
        f.write_str(key_repr)
    }
}

impl From<(Modifiers, KeyCode)> for KeyChord {
    #[inline(always)]
    fn from((mods, key): (Modifiers, KeyCode)) -> Self {
        KeyChord(mods, key)
    }
}

impl From<KeyCode> for KeyChord {
    #[inline(always)]
    fn from(key: KeyCode) -> Self {
        KeyChord(Modifiers::empty(), key)
    }
}

/// Input sequence plugin.
pub struct InputSequencePlugin {
    schedules: Vec<(Interned<dyn ScheduleLabel>, Option<Interned<dyn SystemSet>>)>,
}

impl Default for InputSequencePlugin {
    fn default() -> Self {
        InputSequencePlugin {
            schedules: vec![(Interned(Box::leak(Box::new(Update))), None)],
        }
    }
}

impl Plugin for InputSequencePlugin {
    fn build(&self, app: &mut App) {
        if true || app.world.get_resource::<Input<KeyCode>>().is_some() {
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

        if true || app.world.get_resource::<Input<GamepadButton>>().is_some() {
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
        Self { schedules: vec![] }
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
}

/// Contains the trie for the input sequences.
#[derive(Resource)]
pub struct InputSequenceCache<A, In> {
    trie: Option<Trie<A, InputSequence<A, In>>>,
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

/// The systems run in one of these sets.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum InputSet {
    /// [KeySequences] run in this set.
    Key,
    /// [ButtonSequences] run in this set.
    Button,
}

fn is_modifier(key: KeyCode) -> bool {
    let mods = Modifiers::from(key);
    !mods.is_empty()
}

#[allow(clippy::too_many_arguments)]
fn button_sequence_matcher(
    secrets: Query<&ButtonSequence>,
    time: Res<Time>,
    buttons: Res<Input<GamepadButton>>,
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
    keys: Res<Input<KeyCode>>,
    mut last_keys: Local<Covec<KeyChord, FrameTime>>,
    mut cache: ResMut<InputSequenceCache<KeyChord, ()>>,
    frame_count: Res<FrameCount>,
    mut commands: Commands,
) {
    let mods = Modifiers::from_input(&keys);
    let trie = cache.trie(secrets.iter());
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_seconds(),
    };
    for key_code in keys.get_just_pressed() {
        if is_modifier(*key_code) {
            continue;
        }
        let key = KeyChord(mods, *key_code);
        last_keys.push(key, now.clone());
        let start = last_keys.1[0].clone();
        for seq in consume_input(trie, &mut last_keys.0) {
            if seq
                .time_limit
                .as_ref()
                .map(|limit| (&now - &start).has_timedout(limit))
                .unwrap_or(false)
            {
                // Sequence timed out.
            } else {
                commands.run_system(seq.system_id);
            }
        }
        last_keys.drain1_sync();
    }
}

fn detect_additions<A: Clone + Send + Sync + 'static, In: 'static>(
    secrets: Query<&InputSequence<A, In>, Added<InputSequence<A, In>>>,
    mut cache: ResMut<InputSequenceCache<A, In>>,
) {
    if secrets.iter().next().is_some() {
        cache.trie = None;
    }
}

fn detect_removals<A: Clone + Send + Sync + 'static, In: 'static>(
    mut cache: ResMut<InputSequenceCache<A, In>>,
    mut removals: RemovedComponents<InputSequence<A, In>>,
) {
    if removals.read().next().is_some() {
        cache.trie = None;
    }
}

fn consume_input<'a, K, V>(trie: &'a Trie<K, V>, input: &mut Vec<K>) -> impl Iterator<Item = &'a V>
where
    K: Clone + Eq + Ord,
    // V: Clone,
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

#[cfg(test)]
mod tests {
    use crate::key;
    use crate::KeyChord;

    #[test]
    fn keychord_display() {
        let keychord = KeyChord::from(key!(ctrl - A));
        assert_eq!(format!("{}", keychord), "ctrl-A");
        let keychord = KeyChord::from(key!(ctrl - 1));
        assert_eq!(format!("{}", keychord), "ctrl-1");
        let keychord = KeyChord::from(key!(1));
        assert_eq!(format!("{}", keychord), "1");
    }

    mod simulate_app {
        use bevy:: {
            app::{App, PostUpdate},
            ecs::{
                world::World,
                system::Command
            }
        };
        use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadInfo};
        use bevy::input::{Axis, ButtonInput as Input};
        use bevy::prelude::{
            Commands, Component, Event, EventReader, Gamepad, GamepadAxis, GamepadButton,
            GamepadButtonType, Gamepads, KeyCode,
        };
        use bevy::MinimalPlugins;

        use super::super::*;
        use crate::{TimeLimit, action};

        #[derive(Event, Clone)]
        struct MyEvent;

        #[derive(Component)]
        struct EventSent(u8);

        trait AddCommand {
            fn add(&mut self, command: impl Command);
        }

        impl AddCommand for World {
            fn add(&mut self, command: impl Command) {
                command.apply(self);
            }
        }

        #[test]
        fn one_key() {
            let mut app = new_app();

            app.world.add(KeySequence::new(
                action::send_event(MyEvent),
                [(Modifiers::empty(), KeyCode::KeyA)],
            ));
            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn two_components_one_event() {
            let mut app = new_app();

            app.world.add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA]));
            app.world.add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA]));
            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert_eq!(app.world.query::<&EventSent>().iter(&app.world).count(), 1);
        }

        #[test]
        fn two_presses_two_events() {
            let mut app = new_app();

            app.world.add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA]));
            app.world.add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyB]));
            press_key(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert_eq!(app.world.query::<&EventSent>().iter(&app.world).count(), 2);
        }

        #[test]
        fn two_keycodes_match_first() {
            let mut app = new_app();

            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB]));
            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyC]));

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn match_short_seq() {
            let mut app = new_app();

            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB]));
            app.world.add(KeySequence::new(
                action::send_event(MyEvent),
                [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC],
            ));

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert_eq!(
                app.world
                    .query::<&EventSent>()
                    .iter(&app.world)
                    .next()
                    .map(|x| x.0)
                    .unwrap(),
                1 // .is_some()
            );

            clear_just_pressed(&mut app, KeyCode::KeyB);
            press_key(&mut app, KeyCode::KeyC);
            app.update();
            assert_eq!(
                app.world
                    .query::<&EventSent>()
                    .iter(&app.world)
                    .next()
                    .map(|x| x.0)
                    .unwrap(),
                2 // .is_some()
            );

            clear_just_pressed(&mut app, KeyCode::KeyC);
            press_key(&mut app, KeyCode::KeyD);
            app.update();
            assert_eq!(
                app.world
                    .query::<&EventSent>()
                    .iter(&app.world)
                    .next()
                    .map(|x| x.0)
                    .unwrap(),
                2
            );
        }

        #[test]
        fn two_keycodes_match_second() {
            let mut app = new_app();

            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB]));
            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyC]));

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyC);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn two_any_patterns() {
            let mut app = new_app();

            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB]));
            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyC]));
            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyD]));
            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn two_any_patterns_match_2nd() {
            let mut app = new_app();

            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB]));
            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyC]));
            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyD]));
            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyD);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn two_keycodes() {
            let mut app = new_app();

            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB]));

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn delete_sequences_if_pressed_incorrect_key() {
            let mut app = new_app();

            app.world.add(KeySequence::new(
                action::send_event(MyEvent),
                [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC],
            ));

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyB);
            press_key(&mut app, KeyCode::KeyD);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());
        }

        #[test]
        fn game_pad_button() {
            let mut app = new_app();

            app.world.send_event(GamepadConnectionEvent::new(
                Gamepad::new(1),
                GamepadConnection::Connected(GamepadInfo {
                    name: "".to_string(),
                }),
            ));
            app.world.add(ButtonSequence::new(
                action::send_event_with_input(|_| MyEvent),
                [
                    GamepadButtonType::North,
                    GamepadButtonType::East,
                    GamepadButtonType::South,
                ],
            ));
            app.update();

            press_pad_button(&mut app, GamepadButtonType::North);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed_pad_button(&mut app, GamepadButtonType::North);
            press_pad_button(&mut app, GamepadButtonType::East);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed_pad_button(&mut app, GamepadButtonType::East);
            press_pad_button(&mut app, GamepadButtonType::South);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn multiple_inputs() {
            let mut app = new_app();
            app.world.send_event(GamepadConnectionEvent::new(
                Gamepad::new(1),
                GamepadConnection::Connected(GamepadInfo {
                    name: "".to_string(),
                }),
            ));
            // This is no longer possible right now. We could introduce a
            // KeyButtonSequence mixture struct would allow it.
            // app.world.add(KeySequence::new(
            //     action::send_event(MyEvent),
            //     [
            //         (KeyCode::KeyA),
            //         (KeyCode::KeyB),
            //         (KeyCode::KeyC) | Act::PadButton(GamepadButtonType::North.into()),
            //         (GamepadButtonType::C.into()),
            //     ],
            // ));
            app.world.add(KeySequence::new(
                action::send_event(MyEvent),
                [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyX],
            ));

            app.world.add(ButtonSequence::new(
                action::send_event_with_input(|_| MyEvent),
                [GamepadButtonType::North, GamepadButtonType::C],
            ));
            app.update();

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyB);
            press_pad_button(&mut app, GamepadButtonType::North);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed_pad_button(&mut app, GamepadButtonType::North);
            press_pad_button(&mut app, GamepadButtonType::C);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn timeout_1frame() {
            let mut app = new_app();

            app.world.add(
                KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB])
                    .time_limit(TimeLimit::Frames(1)),
            );

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            app.update();

            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());
        }

        #[test]
        fn test_modifier() {
            let mut app = new_app();

            app.world
                .add(KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB]));

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            app.update();

            press_key(&mut app, KeyCode::ControlLeft);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());
            release(&mut app, KeyCode::ControlLeft);
            app.update();

            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn no_timeout_1frame() {
            let mut app = new_app();

            app.world.add(
                KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB])
                    .time_limit(TimeLimit::Frames(2)),
            );

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            app.update();

            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_some());
        }

        #[test]
        fn timeout_3frames() {
            let mut app = new_app();

            app.world.add(
                KeySequence::new(action::send_event(MyEvent), [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC])
                    .time_limit(TimeLimit::Frames(2)),
            );

            press_key(&mut app, KeyCode::KeyA);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyA);
            app.update();

            press_key(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            clear_just_pressed(&mut app, KeyCode::KeyB);
            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());

            app.update();
            assert!(app
                .world
                .query::<&EventSent>()
                .iter(&app.world)
                .next()
                .is_none());
        }

        fn press_key(app: &mut App, key: KeyCode) {
            app.world.resource_mut::<Input<KeyCode>>().press(key);
        }

        fn clear_just_pressed(app: &mut App, key: KeyCode) {
            app.world
                .resource_mut::<Input<KeyCode>>()
                .clear_just_pressed(key);
        }

        fn release(app: &mut App, key: KeyCode) {
            app.world.resource_mut::<Input<KeyCode>>().release(key);
        }

        fn press_pad_button(app: &mut App, game_button: GamepadButtonType) {
            app.world
                .resource_mut::<Input<GamepadButton>>()
                .press(GamepadButton::new(Gamepad::new(1), game_button))
        }

        fn clear_just_pressed_pad_button(app: &mut App, game_button: GamepadButtonType) {
            app.world
                .resource_mut::<Input<GamepadButton>>()
                .clear_just_pressed(GamepadButton::new(Gamepad::new(1), game_button));
        }

        fn read(
            mut commands: Commands,
            mut er: EventReader<MyEvent>,
            mut query: Query<&mut EventSent>,
        ) {
            for _ in er.read() {
                match query.get_single_mut() {
                    Ok(ref mut event_sent) => {
                        event_sent.0 += 1;
                    }
                    _ => {
                        commands.spawn(EventSent(1));
                    }
                }
            }
        }

        fn new_app() -> App {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins);
            // app.add_plugins(DefaultPlugins);
            app.add_plugins(InputSequencePlugin::default());
            app.add_systems(PostUpdate, read);
            app.add_event::<MyEvent>();
            app.init_resource::<Gamepads>();
            app.init_resource::<Input<GamepadButton>>();
            app.init_resource::<Input<GamepadAxis>>();
            app.init_resource::<Axis<GamepadButton>>();
            app.init_resource::<Axis<GamepadAxis>>();
            app.init_resource::<Input<KeyCode>>();
            app
        }
    }
}
