#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.2.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
use bevy::app::{App, Update};
use bevy::core::FrameCount;
use bevy::ecs::schedule::Condition;
use bevy::prelude::{
    Added, Event, EventWriter, GamepadButton, Input, IntoSystemConfigs, KeyCode, Local, Query,
    RemovedComponents, Res, ResMut, Resource,
};
use bevy::time::Time;
use std::collections::HashMap;
use trie_rs::map::{Trie, TrieBuilder};

pub use crate::act::{Act, ActPattern};
pub use crate::input_sequence::InputSequence;
pub use crate::time_limit::TimeLimit;

pub use keyseq::{
    bevy::{pkey as key, pkeyseq as keyseq},
    Modifiers,
};

mod act;
mod covec;
mod frame_time;
mod input_sequence;
mod time_limit;

use covec::Covec;
use frame_time::FrameTime;

/// App extension trait
pub trait AddInputSequenceEvent {
    /// Setup event `E` so that it may fire when a component `InputSequence<E>` is
    /// present in the app.
    fn add_input_sequence_event<E: Event + Clone>(&mut self) -> &mut App;

    /// Setup event `E` so that it may fire when a component `InputSequence<E>` is
    /// present and the condition is met.
    fn add_input_sequence_event_run_if<E: Event + Clone, M>(
        &mut self,
        condition: impl Condition<M>,
    ) -> &mut App;
}

#[derive(Resource)]
struct InputSequenceCache<E> {
    trie: Option<Trie<ActPattern, InputSequence<E>>>,
}

impl<E: Event + Clone> InputSequenceCache<E> {
    pub(crate) fn trie(
        &mut self,
        sequences: &Query<&InputSequence<E>>,
    ) -> &Trie<ActPattern, InputSequence<E>> {
        self.trie.get_or_insert_with(|| {
            let mut builder = TrieBuilder::new();
            for sequence in sequences.iter() {
                builder.push(sequence.acts.clone(), sequence.clone());
            }
            builder.build()
        })
    }
}

impl<E> Default for InputSequenceCache<E> {
    fn default() -> Self {
        Self { trie: None }
    }
}

impl AddInputSequenceEvent for App {
    #[inline(always)]
    fn add_input_sequence_event<E: Event + Clone>(&mut self) -> &mut App {
        self.init_resource::<InputSequenceCache<E>>()
            .add_event::<E>()
            .add_systems(
                Update,
                (
                    detect_removals::<E>,
                    detect_additions::<E>,
                    input_sequence_matcher::<E>,
                )
                    .chain(),
            )
    }

    fn add_input_sequence_event_run_if<E: Event + Clone, M>(
        &mut self,
        condition: impl Condition<M>,
    ) -> &mut App {
        self.init_resource::<InputSequenceCache<E>>()
            .add_event::<E>()
            .add_systems(
                Update,
                (
                    detect_removals::<E>,
                    detect_additions::<E>,
                    input_sequence_matcher::<E>,
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
fn input_sequence_matcher<E: Event + Clone>(
    mut writer: EventWriter<E>,
    secrets: Query<&InputSequence<E>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    mut last_keys: Local<Covec<ActPattern, FrameTime>>,
    mut last_buttons: Local<HashMap<usize, Covec<ActPattern, FrameTime>>>,
    mut cache: ResMut<InputSequenceCache<E>>,
    frame_count: Res<FrameCount>,
) {
    let mods = Modifiers::from_input(&keys);
    let trie = cache.trie(&secrets);
    let now = FrameTime {
        frame: frame_count.0,
        time: time.elapsed_seconds(),
    };
    for key_code in keys.get_just_pressed() {
        if is_modifier(*key_code) {
            continue;
        }
        let key = Act::KeyChord(mods, *key_code);
        last_keys.push(ActPattern::One(key), now.clone());
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
    for button in buttons.get_just_pressed() {
        let pad_buttons = match last_buttons.get_mut(&button.gamepad.id) {
            Some(x) => x,
            None => {
                last_buttons.insert(button.gamepad.id, Covec::default());
                last_buttons.get_mut(&button.gamepad.id).unwrap()
            }
        };

        pad_buttons.push(ActPattern::One(button.button_type.into()), now.clone());
        for seq in consume_input(trie, &mut pad_buttons.0) {
            writer.send(seq.event);
        }
        pad_buttons.drain1_sync();
    }
}

fn detect_additions<E: Event + Clone>(
    secrets: Query<&InputSequence<E>, Added<InputSequence<E>>>,
    mut cache: ResMut<InputSequenceCache<E>>,
) {
    if secrets.iter().next().is_some() {
        cache.trie = None;
    }
}

fn detect_removals<E: Event>(
    mut cache: ResMut<InputSequenceCache<E>>,
    mut removals: RemovedComponents<InputSequence<E>>,
) {
    if removals.read().next().is_some() {
        cache.trie = None;
    }
}

fn consume_input<E: Event + Clone>(
    trie: &Trie<ActPattern, InputSequence<E>>,
    input: &mut Vec<ActPattern>,
) -> impl Iterator<Item = InputSequence<E>> {
    let mut result = vec![];
    for i in 0..input.len() {
        if let Some(seq) = trie.exact_match(&input[i..]) {
            result.push(seq.clone());
        } else if trie.is_prefix(&input[i..]) {
            let _ = input.drain(0..i);
            return result.into_iter();
        }
    }
    input.clear();
    result.into_iter()
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, PostUpdate};
    use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadInfo};
    use bevy::input::{Axis, Input};
    use bevy::prelude::{
        Commands, Component, Event, EventReader, Gamepad, GamepadAxis, GamepadButton,
        GamepadButtonType, Gamepads, KeyCode,
    };
    use bevy::MinimalPlugins;

    use super::*;
    use crate::input_sequence::InputSequence;
    use crate::TimeLimit;

    #[derive(Event, Clone)]
    struct MyEvent;

    #[derive(Component)]
    struct EventSent;

    #[test]
    fn one_key() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(
            MyEvent,
            [(Modifiers::empty(), KeyCode::A)],
        ));
        press_key(&mut app, KeyCode::A);
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

        app.world.spawn(InputSequence::new(
            MyEvent,
            [KeyCode::A],
        ));
        app.world.spawn(InputSequence::new(
            MyEvent,
            [KeyCode::A],
        ));
        press_key(&mut app, KeyCode::A);
        app.update();
        assert_eq!(app
                   .world
                   .query::<&EventSent>()
                   .iter(&app.world)
                   .count(),
                   1);

    }

    #[test]
    fn two_presses_two_events() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(
            MyEvent,
            [KeyCode::A],
        ));
        app.world.spawn(InputSequence::new(
            MyEvent,
            [KeyCode::B],
        ));
        press_key(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::B);
        app.update();
        assert_eq!(app
                   .world
                   .query::<&EventSent>()
                   .iter(&app.world)
                   .count(),
                   2);

    }

    #[test]
    fn two_keycodes_match_first() {
        let mut app = new_app();

        app.world
            .spawn(InputSequence::new(MyEvent, [ActPattern::from(KeyCode::A), Act::from(KeyCode::B) | Act::from(KeyCode::C)]));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::B);
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
            .spawn(InputSequence::new(MyEvent, [ActPattern::from(KeyCode::A), Act::from(KeyCode::B) | Act::from(KeyCode::C)]));
        app.world
            .spawn(InputSequence::new(MyEvent, [ActPattern::from(KeyCode::A), Act::from(KeyCode::C) | Act::from(KeyCode::D)]));
        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::B);
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
            .spawn(InputSequence::new(MyEvent, [ActPattern::from(KeyCode::A), Act::from(KeyCode::B) | Act::from(KeyCode::C)]));
        app.world
            .spawn(InputSequence::new(MyEvent, [ActPattern::from(KeyCode::A), Act::from(KeyCode::C) | Act::from(KeyCode::D)]));
        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::D);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_some());
    }

    #[test]
    fn two_keycodes_match_second() {
        let mut app = new_app();

        app.world
            .spawn(InputSequence::new(MyEvent, [ActPattern::from(KeyCode::A), Act::from(KeyCode::B) | Act::from(KeyCode::C)]));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::C);
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
            .spawn(InputSequence::new(MyEvent, [KeyCode::A, KeyCode::B]));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::B);
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

        app.world.spawn(InputSequence::new(
            MyEvent,
            [KeyCode::A, KeyCode::B, KeyCode::C],
        ));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::B);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::B);
        press_key(&mut app, KeyCode::D);
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
        app.world.spawn(InputSequence::new(
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

    //
    // #[test]
    // fn multiple_inputs() {
    //     let mut app = new_app();
    //     app.world.send_event(GamepadConnectionEvent::new(
    //         Gamepad::new(1),
    //         GamepadConnection::Connected(GamepadInfo {
    //             name: "".to_string(),
    //         }),
    //     ));
    //     app.world.spawn(InputSequence::new(
    //         MyEvent,
    //         [
    //             Act::key(KeyCode::A),
    //             Act::key(KeyCode::B),
    //             Act::key(KeyCode::C) | Act::PadButton(GamepadButtonType::North.into()),
    //             Act::PadButton(GamepadButtonType::C.into()),
    //         ],
    //     ));
    //     app.update();

    //     press_key(&mut app, KeyCode::A);
    //     app.update();
    //     assert!(app
    //         .world
    //         .query::<&EventSent>()
    //         .iter(&app.world)
    //         .next()
    //         .is_none());

    //     clear_just_pressed(&mut app, KeyCode::A);
    //     press_key(&mut app, KeyCode::B);
    //     app.update();
    //     assert!(app
    //         .world
    //         .query::<&EventSent>()
    //         .iter(&app.world)
    //         .next()
    //         .is_none());

    //     clear_just_pressed(&mut app, KeyCode::B);
    //     press_pad_button(&mut app, GamepadButtonType::North);
    //     app.update();
    //     assert!(app
    //         .world
    //         .query::<&EventSent>()
    //         .iter(&app.world)
    //         .next()
    //         .is_none());

    //     clear_just_pressed_pad_button(&mut app, GamepadButtonType::North);
    //     press_pad_button(&mut app, GamepadButtonType::C);
    //     app.update();
    //     assert!(app
    //         .world
    //         .query::<&EventSent>()
    //         .iter(&app.world)
    //         .next()
    //         .is_some());
    // }

    #[test]
    fn timeout_1frame() {
        let mut app = new_app();

        app.world.spawn(
            InputSequence::new(MyEvent, [KeyCode::A, KeyCode::B]).time_limit(TimeLimit::Frames(1)),
        );

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        app.update();

        press_key(&mut app, KeyCode::B);
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
            .spawn(InputSequence::new(MyEvent, [KeyCode::A, KeyCode::B]));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
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

        press_key(&mut app, KeyCode::B);
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
            InputSequence::new(MyEvent, [KeyCode::A, KeyCode::B]).time_limit(TimeLimit::Frames(2)),
        );

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        app.update();

        press_key(&mut app, KeyCode::B);
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
            InputSequence::new(MyEvent, [KeyCode::A, KeyCode::B, KeyCode::C])
                .time_limit(TimeLimit::Frames(2)),
        );

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        app.update();

        press_key(&mut app, KeyCode::B);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());

        clear_just_pressed(&mut app, KeyCode::B);
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

    fn read(mut commands: Commands, mut er: EventReader<MyEvent>) {
        for _ in er.read() {
            commands.spawn(EventSent);
        }
    }

    fn new_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(PostUpdate, read);
        app.add_input_sequence_event::<MyEvent>();
        app.init_resource::<Gamepads>();
        app.init_resource::<Input<GamepadButton>>();
        app.init_resource::<Input<GamepadAxis>>();
        app.init_resource::<Axis<GamepadButton>>();
        app.init_resource::<Axis<GamepadAxis>>();
        app.init_resource::<Input<KeyCode>>();
        app
    }
}
