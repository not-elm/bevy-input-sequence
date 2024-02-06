#![doc(html_root_url = "https://docs.rs/bevy-input-sequence/0.2.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
use bevy::app::{App, Update};
use bevy::ecs::system::SystemParam;
use bevy::input::Input;
use bevy::prelude::{
    Commands, Entity, Event, EventWriter, GamepadButton, IntoSystemConfigs, KeyCode, Query, Res,
};
use bevy::time::Time;

pub use crate::act::Act;
pub use crate::input_sequence::InputSequence;
use crate::sequence_reader::SequenceReader;
pub use crate::timeout::TimeLimit;

pub use bevy_input_sequence_macro::{key, keyseq};

mod act;
mod input_sequence;
mod sequence_reader;
mod timeout;

/// Convenient glob imports
pub mod prelude {
    pub use crate::act::{Act, Modifiers};
    pub use crate::input_sequence::InputSequence;
    pub use crate::timeout::TimeLimit;
    pub use crate::AddInputSequenceEvent;
    pub use bevy_input_sequence_macro::{key, keyseq};
}

/// App extension trait
pub trait AddInputSequenceEvent {
    /// Setup event `E` so that it may fire when a component `InputSequence` is
    /// present in the app.
    fn add_input_sequence_event<E: Event + Clone>(&mut self) -> &mut App;
}

impl AddInputSequenceEvent for App {
    #[inline(always)]
    fn add_input_sequence_event<E: Event + Clone>(&mut self) -> &mut App {
        self.add_event::<E>()
            .add_systems(Update, (input_system::<E>, start_input_system::<E>).chain())
    }
}

#[derive(SystemParam)]
struct InputParams<'w> {
    pub key: Res<'w, Input<KeyCode>>,
    pub button_inputs: Res<'w, Input<GamepadButton>>,
}

fn start_input_system<E: Event + Clone>(
    mut commands: Commands,
    mut ew: EventWriter<E>,
    secrets: Query<&InputSequence<E>>,
    inputs: InputParams,
) {
    for seq in secrets.iter() {
        let Some(input) = seq.first_input() else {
            continue;
        };
        let (yes, context) = input.just_inputted(&inputs, &None);
        if yes {
            if seq.one_key() {
                ew.send(seq.event.clone());
            } else {
                commands.spawn(SequenceReader::new(seq.clone(), 1, context));
            }
        }
    }
}

fn input_system<E: Event + Clone>(
    mut commands: Commands,
    mut ew: EventWriter<E>,
    mut key_seq: Query<(Entity, &mut SequenceReader<E>)>,
    time: Res<Time>,
    inputs: InputParams,
) {
    for (seq_entity, mut seq) in key_seq.iter_mut() {
        let Some(next_input) = seq.next_input() else {
            // eprintln!("no more input");
            commands.entity(seq_entity).despawn();
            continue;
        };

        if next_input.just_inputted(&inputs, &seq.context).0 {
            seq.next_act();
            if seq.is_last() {
                // eprintln!("send event");
                commands.entity(seq_entity).despawn();
                ew.send(seq.event());
            }
        } else if seq.just_other_inputted(&inputs, &next_input) || seq.timedout(&time) {
            // eprintln!("time_limit or other input");
            commands.entity(seq_entity).despawn();
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Update};
    use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadInfo};
    use bevy::input::{Axis, Input};
    use bevy::prelude::{
        Commands, Component, Event, EventReader, Gamepad, GamepadAxis, GamepadButton,
        GamepadButtonType, Gamepads, IntoSystemConfigs, KeyCode,
    };
    use bevy::MinimalPlugins;

    use crate::act::Act;
    use crate::input_sequence::InputSequence;
    use crate::prelude::TimeLimit;
    use crate::sequence_reader::SequenceReader;
    use crate::{input_system, start_input_system};

    #[derive(Event, Clone)]
    struct MyEvent;

    #[derive(Component)]
    struct EventSent;

    #[test]
    fn one_key() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(MyEvent, [KeyCode::A]));
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
        assert!(app
            .world
            .query::<&SequenceReader<MyEvent>>()
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

    #[test]
    fn multiple_inputs() {
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
                Act::Key(KeyCode::A),
                Act::Key(KeyCode::B),
                Act::Key(KeyCode::C) | Act::PadButton(GamepadButtonType::North),
                Act::PadButton(GamepadButtonType::C),
            ],
        ));
        app.update();

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
        assert!(app
            .world
            .query::<&SequenceReader<MyEvent>>()
            .iter(&app.world)
            .next()
            .is_none());
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
        assert!(app
            .world
            .query::<&SequenceReader<MyEvent>>()
            .iter(&app.world)
            .next()
            .is_none());
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
        assert!(app
            .world
            .query::<&SequenceReader<MyEvent>>()
            .iter(&app.world)
            .next()
            .is_some());

        clear_just_pressed(&mut app, KeyCode::B);
        app.update();
        assert!(app
            .world
            .query::<&EventSent>()
            .iter(&app.world)
            .next()
            .is_none());
        assert!(app
            .world
            .query::<&SequenceReader<MyEvent>>()
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
        assert!(app
            .world
            .query::<&SequenceReader<MyEvent>>()
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
        app.add_systems(Update, read);
        app.add_event::<MyEvent>();
        app.add_systems(
            Update,
            (input_system::<MyEvent>, start_input_system::<MyEvent>).chain(),
        );
        app.init_resource::<Gamepads>();
        app.init_resource::<Input<GamepadButton>>();
        app.init_resource::<Input<GamepadAxis>>();
        app.init_resource::<Axis<GamepadButton>>();
        app.init_resource::<Axis<GamepadAxis>>();
        app.init_resource::<Input<KeyCode>>();

        app
    }
}
