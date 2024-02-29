#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.3.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
use bevy::app::{App, Update};
use bevy::core::FrameCount;
use bevy::ecs::schedule::Condition;
use bevy::prelude::{
    Added, ButtonInput as Input, Event, EventWriter, Gamepad, GamepadButton, GamepadButtonType,
    IntoSystemConfigs, KeyCode, Local, Query, RemovedComponents, Res, ResMut, Resource};
use bevy::time::Time;
use std::collections::HashMap;
use trie_rs::map::{Trie, TrieBuilder};

pub use crate::input_sequence::{ButtonSequence, InputSequence, KeySequence};
pub use crate::time_limit::TimeLimit;

pub use keyseq::{
    bevy::{pkey as key, pkeyseq as keyseq},
    Modifiers,
};

mod covec;
mod frame_time;
mod input_sequence;
mod time_limit;

use covec::Covec;
use frame_time::FrameTime;

/// Represents a key chord, i.e., a set of modifiers and a key code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyChord(pub Modifiers, pub KeyCode);

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
/// A gamepad event includes what gamepad the event came from.
pub trait GamepadEvent: Event {
    /// Return gamepad if available.
    fn gamepad(&self) -> Option<Gamepad>;
    /// Set gamepad.
    fn set_gamepad(&mut self, gamepad: Gamepad);
}

/// App extension trait
pub trait AddInputSequenceEvent {
    /// Setup event `E` so that it may fire when a component `KeySequence<E>` is
    /// present in the app.
    fn add_key_sequence_event<E: Event + Clone>(&mut self) -> &mut App;

    /// Setup event `E` so that it may fire when a component `ButtonSequence<E>` is
    /// present in the app.
    fn add_button_sequence_event<E: GamepadEvent + Clone>(&mut self) -> &mut App;

    /// Setup event `E` so that it may fire when a component `KeySequence<E>` is
    /// present and the condition is met.
    fn add_key_sequence_event_run_if<E: Event + Clone, M>(
        &mut self,
        condition: impl Condition<M>,
    ) -> &mut App;

    /// Setup event `E` so that it may fire when a component `ButtonSequence<E>` is
    /// present in the app.
    fn add_button_sequence_event_run_if<E: GamepadEvent + Clone, M>(
        &mut self,
        condition: impl Condition<M>,
    ) -> &mut App;
}

#[derive(Resource)]
struct InputSequenceCache<E, A> {
    trie: Option<Trie<A, InputSequence<E, A>>>,
}

impl<E, A> InputSequenceCache<E, A>
where
    E: Event + Clone,
    A: Ord + Clone + Send + Sync + 'static,
{
    pub(crate) fn build_trie<'a>(
        &mut self,
        sequences: impl Iterator<Item = &'a InputSequence<E, A>>,
    ) {
        let mut builder = TrieBuilder::new();
        for sequence in sequences {
            builder.push(sequence.acts.clone(), sequence.clone());
        }
        self.trie = Some(builder.build())
    }
}

impl<E, A> Default for InputSequenceCache<E, A> {
    fn default() -> Self {
        Self { trie: None }
    }
}

impl AddInputSequenceEvent for App {
    #[inline(always)]
    fn add_key_sequence_event<E: Event + Clone>(&mut self) -> &mut App {
        self.add_event::<E>()
            .init_resource::<InputSequenceCache<E, KeyChord>>()
            .add_systems(
                Update,
                (
                    detect_removals::<E, KeyChord>,
                    detect_additions::<E, KeyChord>,
                    key_sequence_matcher::<E>,
                )
                    .chain(),
            )
    }

    fn add_key_sequence_event_run_if<E: Event + Clone, M>(
        &mut self,
        condition: impl Condition<M>,
    ) -> &mut App {
        self.init_resource::<InputSequenceCache<E, KeyChord>>()
            .add_event::<E>()
            .add_systems(
                Update,
                (
                    detect_removals::<E, KeyChord>,
                    detect_additions::<E, KeyChord>,
                    key_sequence_matcher::<E>,
                )
                    .chain()
                    .run_if(condition),
            )
    }

    fn add_button_sequence_event<E: GamepadEvent + Clone>(&mut self) -> &mut App {
        self.add_event::<E>()
            .init_resource::<InputSequenceCache<E, GamepadButtonType>>()
            .add_systems(
                Update,
                (
                    detect_removals::<E, GamepadButtonType>,
                    detect_additions::<E, GamepadButtonType>,
                    button_sequence_matcher::<E>,
                )
                    .chain(),
            )
    }

    fn add_button_sequence_event_run_if<E: GamepadEvent + Clone, M>(
        &mut self,
        condition: impl Condition<M>,
    ) -> &mut App {
        self.add_event::<E>()
            .init_resource::<InputSequenceCache<E, GamepadButtonType>>()
            .add_systems(
                Update,
                (
                    detect_removals::<E, GamepadButtonType>,
                    detect_additions::<E, GamepadButtonType>,
                    button_sequence_matcher::<E>,
                )
                    .chain()
                    .run_if(condition),
            )
    }
}

fn is_modifier(key: KeyCode) -> bool {
    let mods = Modifiers::from(key);
    !mods.is_empty()
}

#[allow(clippy::too_many_arguments)]
fn button_sequence_matcher<E: GamepadEvent + Clone>(
    mut writer: EventWriter<E>,
    secrets: Query<&ButtonSequence<E>>,
    time: Res<Time>,
    buttons: Res<Input<GamepadButton>>,
    mut last_buttons: Local<HashMap<usize, Covec<GamepadButtonType, FrameTime>>>,
    mut cache: ResMut<InputSequenceCache<E, GamepadButtonType>>,
    frame_count: Res<FrameCount>,
) {
    if cache.trie.is_none() {
        cache.build_trie(secrets.iter().map(|c| &c.0));
    }
    let trie = &cache.trie.as_ref().unwrap();
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
        for mut seq in consume_input(trie, &mut pad_buttons.0) {
            if seq
                .time_limit
                .map(|limit| (&now - &start).has_timedout(&limit))
                .unwrap_or(false)
            {
                // Sequence timed out.
            } else {
                seq.event.set_gamepad(button.gamepad);
                writer.send(seq.event);
            }
        }
        pad_buttons.drain1_sync();
    }
}

#[allow(clippy::too_many_arguments)]
fn key_sequence_matcher<E: Event + Clone>(
    mut writer: EventWriter<E>,
    secrets: Query<&KeySequence<E>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut last_keys: Local<Covec<KeyChord, FrameTime>>,
    mut cache: ResMut<InputSequenceCache<E, KeyChord>>,
    frame_count: Res<FrameCount>,
) {
    let mods = Modifiers::from_input(&keys);
    if cache.trie.is_none() {
        cache.build_trie(secrets.iter());
    }
    let trie = &cache.trie.as_ref().unwrap();
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
                .map(|limit| (&now - &start).has_timedout(&limit))
                .unwrap_or(false)
            {
                // Sequence timed out.
            } else {
                writer.send(seq.event);
            }
        }
        last_keys.drain1_sync();
    }
}

fn detect_additions<E: Event + Clone, A: Clone + Send + Sync + 'static>(
    secrets: Query<&InputSequence<E, A>, Added<InputSequence<E, A>>>,
    mut cache: ResMut<InputSequenceCache<E, A>>,
) {
    if secrets.iter().next().is_some() {
        cache.trie = None;
    }
}

fn detect_removals<E: Event, A: Clone + Send + Sync + 'static>(
    mut cache: ResMut<InputSequenceCache<E, A>>,
    mut removals: RemovedComponents<InputSequence<E, A>>,
) {
    if removals.read().next().is_some() {
        cache.trie = None;
    }
}

fn consume_input<K, V>(trie: &Trie<K, V>, input: &mut Vec<K>) -> impl Iterator<Item = V>
where
    K: Clone + Eq + Ord,
    V: Clone,
{
    let mut result = vec![];
    let mut min_prefix = None;
    for i in 0..input.len() {
        if let Some(seq) = trie.exact_match(&input[i..]) {
            result.push(seq.clone());
        }
        if min_prefix.is_none() && trie.is_prefix(&input[i..]) {
            min_prefix = Some(i);
            // let _ = input.drain(0..i);
            // return result.into_iter();
        }
    }
    match min_prefix {
        Some(i) => {let _ = input.drain(0..i);}
        None => { input.clear(); }
    }
    result.into_iter()
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, PostUpdate};
    use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadInfo};
    use bevy::input::{Axis, ButtonInput as Input};
    use bevy::prelude::{
        Commands, Component, Event, EventReader, Gamepad, GamepadAxis, GamepadButton,
        GamepadButtonType, Gamepads, KeyCode,
    };
    use bevy::MinimalPlugins;

    use super::*;
    use crate::TimeLimit;

    #[derive(Event, Clone)]
    struct MyEvent;

    impl GamepadEvent for MyEvent {
        fn gamepad(&self) -> Option<Gamepad> {
            None
        }

        fn set_gamepad(&mut self, _gamepad: Gamepad) {}
    }

    #[derive(Component)]
    struct EventSent(u8);

    #[test]
    fn one_key() {
        let mut app = new_app();

        app.world.spawn(KeySequence::new(
            MyEvent,
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

        app.world.spawn(KeySequence::new(MyEvent, [KeyCode::KeyA]));
        app.world.spawn(KeySequence::new(MyEvent, [KeyCode::KeyA]));
        press_key(&mut app, KeyCode::KeyA);
        app.update();
        assert_eq!(app.world.query::<&EventSent>().iter(&app.world).count(), 1);
    }

    #[test]
    fn two_presses_two_events() {
        let mut app = new_app();

        app.world.spawn(KeySequence::new(MyEvent, [KeyCode::KeyA]));
        app.world.spawn(KeySequence::new(MyEvent, [KeyCode::KeyB]));
        press_key(&mut app, KeyCode::KeyA);
        press_key(&mut app, KeyCode::KeyB);
        app.update();
        assert_eq!(app.world.query::<&EventSent>().iter(&app.world).count(), 2);
    }

    #[test]
    fn two_keycodes_match_first() {
        let mut app = new_app();

        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB]));
        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyC]));

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
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB]));
        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB,
                                              KeyCode::KeyC]));

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
        assert_eq!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .map(|x| x.0)
            .unwrap(),
                1
            // .is_some()
        );

        clear_just_pressed(&mut app, KeyCode::KeyB);
        press_key(&mut app, KeyCode::KeyC);
        app.update();
        assert_eq!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .map(|x| x.0)
            .unwrap(),
                    2
            // .is_some()
            );

        clear_just_pressed(&mut app, KeyCode::KeyC);
        press_key(&mut app, KeyCode::KeyD);
        app.update();
        assert_eq!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .map(|x| x.0)
            .unwrap(),
                    2);
    }

    #[test]
    fn two_keycodes_match_second() {
        let mut app = new_app();

        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB]));
        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyC]));

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
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB]));
        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyC]));
        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyD]));
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
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB]));
        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyC]));
        app.world
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyD]));
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
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB]));

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

        app.world.spawn(KeySequence::new(
            MyEvent,
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
        app.world.spawn(ButtonSequence::new(
            MyEvent,
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
        // app.world.spawn(KeySequence::new(
        //     MyEvent,
        //     [
        //         (KeyCode::KeyA),
        //         (KeyCode::KeyB),
        //         (KeyCode::KeyC) | Act::PadButton(GamepadButtonType::North.into()),
        //         (GamepadButtonType::C.into()),
        //     ],
        // ));
        app.world.spawn(KeySequence::new(
            MyEvent,
            [
                KeyCode::KeyA,
                KeyCode::KeyB,
                KeyCode::KeyX,
            ],
        ));

        app.world.spawn(ButtonSequence::new(
            MyEvent,
            [
                GamepadButtonType::North,
                GamepadButtonType::C,
            ],
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

        app.world.spawn(
            KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB])
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
            .spawn(KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB]));

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

        app.world.spawn(
            KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB])
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

        app.world.spawn(
            KeySequence::new(MyEvent, [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC])
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

    fn read(mut commands: Commands,
            mut er: EventReader<MyEvent>,
            mut query: Query<&mut EventSent>) {
        for _ in er.read() {
            match query.get_single_mut() {
                Ok(ref mut event_sent) => { event_sent.0 += 1; },
                _ => { commands.spawn(EventSent(1)); }
            }
        }
    }

    fn new_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(PostUpdate, read);
        app.add_key_sequence_event::<MyEvent>();
        app.add_button_sequence_event::<MyEvent>();
        app.init_resource::<Gamepads>();
        app.init_resource::<Input<GamepadButton>>();
        app.init_resource::<Input<GamepadAxis>>();
        app.init_resource::<Axis<GamepadButton>>();
        app.init_resource::<Axis<GamepadAxis>>();
        app.init_resource::<Input<KeyCode>>();
        app
    }
}
