use bevy::app::{App, FixedUpdate};
use bevy::ecs::system::SystemParam;
use bevy::input::{Axis, Input};
use bevy::prelude::{Commands, Entity, Event, EventWriter, GamepadAxis, GamepadButton, IntoSystemConfigs, KeyCode, Query, Res};
use bevy::time::Time;

use crate::prelude::InputSequence;
use crate::sequence::KeySequence;

mod sequence;
mod secret;
mod timeout;
mod input;


pub mod prelude {
    pub use crate::AppSecretCommandEx;
    pub use crate::input::Entry;
    pub use crate::secret::InputSequence;
    pub use crate::timeout::Timeout;
}

pub trait AppSecretCommandEx {
    fn add_secret_command_event<E: Event + Clone>(&mut self) -> &mut App;
}


impl AppSecretCommandEx for App {
    #[inline(always)]
    fn add_secret_command_event<E: Event + Clone>(&mut self) -> &mut App {
        self
            .add_event::<E>()
            .add_systems(FixedUpdate, (
                input_system::<E>,
                start_input_system::<E>,
            ).chain())
    }
}


#[derive(SystemParam)]
struct InputParams<'w> {
    pub key: Res<'w, Input<KeyCode>>,
    pub button_inputs: Res<'w, Input<GamepadButton>>,
    pub button_axes: Res<'w, Axis<GamepadButton>>,
    pub axes: Res<'w, Axis<GamepadAxis>>,
}


fn start_input_system<E: Event + Clone>(
    mut commands: Commands,
    mut ew: EventWriter<E>,
    secrets: Query<&InputSequence<E>>,
    inputs: InputParams,
) {
    for secret in secrets.iter() {
        let Some(input) = secret.next_input() else { continue; };

        if input.just_inputted(&inputs) {
            if secret.once_key() {
                ew.send(secret.event());
            } else {
                commands.spawn(secret.next_sequence());
            }
        }
    }
}


fn input_system<E: Event + Clone>(
    mut commands: Commands,
    mut ew: EventWriter<E>,
    mut key_seq: Query<(Entity, &mut KeySequence<E>)>,
    time: Res<Time>,
    inputs: InputParams,
) {
    for (seq_entity, mut seq) in key_seq.iter_mut() {
        let Some(next_input) = seq.next_input() else {
            commands.entity(seq_entity).despawn();
            continue;
        };

        if next_input.just_inputted(&inputs) {
            commands.entity(seq_entity).despawn();
            if seq.is_last() {
                ew.send(seq.event());
            } else {
                commands.spawn(seq.next_sequence());
            }
        } else if seq.timeout(&time) || just_other_inputted(&inputs) {
            commands.entity(seq_entity).despawn();
        }
    }
}


fn just_other_inputted(
    inputs: &InputParams
) -> bool {
    if 0 < inputs.key.get_just_pressed().len() {
        return true;
    }

    if 0 < inputs.button_inputs.get_just_pressed().len() {
        return true;
    }

    if inputs.button_axes
        .devices()
        .filter_map(|button| inputs.button_axes.get(*button))
        .any(|axis| 0.01 < axis.abs()) {
        return true;
    }

    inputs.axes
        .devices()
        .filter_map(|pad_axis| inputs.axes.get(*pad_axis))
        .any(|axis| 0.01 < axis.abs())
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Update};
    use bevy::input::{Axis, Input};
    use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadInfo};
    use bevy::MinimalPlugins;
    use bevy::prelude::{Commands, Component, Event, EventReader, Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, Gamepads, IntoSystemConfigs, KeyCode};

    use crate::{input_system, start_input_system};
    use crate::input::{Entry, ToInputs};
    use crate::prelude::Timeout;
    use crate::secret::InputSequence;
    use crate::sequence::KeySequence;

    #[derive(Event, Clone)]
    struct MyEvent;

    #[derive(Component)]
    struct EventSent;


    #[test]
    fn once_key() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(
            MyEvent,
            Timeout::None,
            &[KeyCode::A].to_inputs(),
        ));
        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_some());
    }


    #[test]
    fn two_keycodes() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(
            MyEvent,
            Timeout::None,
            &[KeyCode::A, KeyCode::B].to_inputs(),
        ));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::B);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_some());
    }


    #[test]
    fn delete_sequences_if_pressed_incorrect_key() {
        let mut app = new_app();

        app.world.spawn(InputSequence::from_keycodes(
            MyEvent,
            Timeout::None,
            &[KeyCode::A, KeyCode::B, KeyCode::C],
        ));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        press_key(&mut app, KeyCode::B);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed(&mut app, KeyCode::B);
        press_key(&mut app, KeyCode::D);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());
        assert!(app.world.query::<&KeySequence<MyEvent>>().iter(&app.world).next().is_none());
    }


    #[test]
    fn game_pad_button() {
        let mut app = new_app();

        app.world.send_event(GamepadConnectionEvent::new(Gamepad::new(1), GamepadConnection::Connected(GamepadInfo { name: "".to_string() })));
        app.world.spawn(InputSequence::from_pad_buttons(
            MyEvent,
            Timeout::None,
            &[
                GamepadButtonType::North,
                GamepadButtonType::East,
                GamepadButtonType::South,
            ],
        ));
        app.update();

        press_pad_button(&mut app, GamepadButtonType::North);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed_pad_button(&mut app, GamepadButtonType::North);
        press_pad_button(&mut app, GamepadButtonType::East);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed_pad_button(&mut app, GamepadButtonType::East);
        press_pad_button(&mut app, GamepadButtonType::South);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_some());
    }


    #[test]
    fn game_pad_button_axis() {
        let mut app = new_app();
        app.world.send_event(GamepadConnectionEvent::new(Gamepad::new(1), GamepadConnection::Connected(GamepadInfo { name: "".to_string() })));
        app.world.spawn(InputSequence::from_pad_button_axes(
            MyEvent,
            Timeout::None,
            &[
                GamepadButtonType::RightTrigger,
                GamepadButtonType::LeftTrigger,
                GamepadButtonType::RightTrigger,
            ],
        ));
        app.update();

        pad_button_axis(&mut app, GamepadButtonType::RightTrigger);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_pad_button_axis(&mut app, GamepadButtonType::RightTrigger);
        pad_button_axis(&mut app, GamepadButtonType::LeftTrigger);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_pad_button_axis(&mut app, GamepadButtonType::LeftTrigger);
        pad_button_axis(&mut app, GamepadButtonType::RightTrigger);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_some());
    }


    #[test]
    fn game_pad_axis() {
        let mut app = new_app();
        app.world.send_event(GamepadConnectionEvent::new(Gamepad::new(1), GamepadConnection::Connected(GamepadInfo { name: "".to_string() })));
        app.world.spawn(InputSequence::from_pad_axes(
            MyEvent,
            Timeout::None,
            &[
                GamepadAxisType::LeftStickX,
                GamepadAxisType::RightZ,
                GamepadAxisType::RightStickX,
            ],
        ));
        app.update();

        pad_axis(&mut app, GamepadAxisType::LeftStickX);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_pad_axis(&mut app, GamepadAxisType::LeftStickX);
        pad_axis(&mut app, GamepadAxisType::RightZ);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_pad_axis(&mut app, GamepadAxisType::RightZ);
        pad_axis(&mut app, GamepadAxisType::RightStickX);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_some());
    }


    #[test]
    fn multiple_inputs() {
        let mut app = new_app();
        app.world.send_event(GamepadConnectionEvent::new(Gamepad::new(1), GamepadConnection::Connected(GamepadInfo { name: "".to_string() })));
        app.world.spawn(InputSequence::new(
            MyEvent,
            Timeout::None,
            &[
                Entry::PadAxis(GamepadAxisType::LeftStickX),
                Entry::Key(KeyCode::E) | Entry::PadAxis(GamepadAxisType::LeftStickX),
                Entry::PadAxis(GamepadAxisType::LeftStickX) | Entry::PadButton(GamepadButtonType::North),
                Entry::PadButtonAxis(GamepadButtonType::LeftTrigger)
            ],
        ));
        app.update();

        pad_axis(&mut app, GamepadAxisType::LeftStickX);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_pad_axis(&mut app, GamepadAxisType::LeftStickX);
        press_key(&mut app, KeyCode::E);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed(&mut app, KeyCode::E);
        press_pad_button(&mut app, GamepadButtonType::North);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed_pad_button(&mut app, GamepadButtonType::North);
        pad_button_axis(&mut app, GamepadButtonType::LeftTrigger);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_some());
    }


    #[test]
    fn timeout_1frame() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(
            MyEvent,
            Timeout::from_frame_count(1),
            &[KeyCode::A, KeyCode::B].to_inputs(),
        ));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        app.update();

        press_key(&mut app, KeyCode::B);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());
        assert!(app.world.query::<&KeySequence<MyEvent>>().iter(&app.world).next().is_none());
    }


    #[test]
    fn no_timeout_1frame() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(
            MyEvent,
            Timeout::from_frame_count(2),
            &[KeyCode::A, KeyCode::B].to_inputs(),
        ));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        app.update();

        press_key(&mut app, KeyCode::B);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_some());
        assert!(app.world.query::<&KeySequence<MyEvent>>().iter(&app.world).next().is_none());
    }


    #[test]
    fn timeout_3frames() {
        let mut app = new_app();

        app.world.spawn(InputSequence::new(
            MyEvent,
            Timeout::from_frame_count(2),
            &[KeyCode::A, KeyCode::B, KeyCode::C].to_inputs(),
        ));

        press_key(&mut app, KeyCode::A);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());

        clear_just_pressed(&mut app, KeyCode::A);
        app.update();

        press_key(&mut app, KeyCode::B);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());
        assert!(app.world.query::<&KeySequence<MyEvent>>().iter(&app.world).next().is_some());

        clear_just_pressed(&mut app, KeyCode::B);
        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());
        assert!(app.world.query::<&KeySequence<MyEvent>>().iter(&app.world).next().is_some());

        app.update();
        assert!(app.world.query::<&EventSent>().iter(&app.world).next().is_none());
        assert!(app.world.query::<&KeySequence<MyEvent>>().iter(&app.world).next().is_none());
    }


    fn press_key(app: &mut App, key: KeyCode) {
        app.world.resource_mut::<Input<KeyCode>>().press(key);
    }

    fn clear_just_pressed(app: &mut App, key: KeyCode) {
        app.world.resource_mut::<Input<KeyCode>>().clear_just_pressed(key);
    }


    fn press_pad_button(app: &mut App, game_button: GamepadButtonType) {
        app.world.resource_mut::<Input<GamepadButton>>().press(GamepadButton::new(Gamepad::new(1), game_button))
    }


    fn clear_just_pressed_pad_button(app: &mut App, game_button: GamepadButtonType) {
        app.world.resource_mut::<Input<GamepadButton>>().clear_just_pressed(GamepadButton::new(Gamepad::new(1), game_button));
    }


    fn pad_button_axis(app: &mut App, game_button: GamepadButtonType) {
        app.world.resource_mut::<Axis<GamepadButton>>().set(GamepadButton::new(Gamepad::new(1), game_button), 1.0);
    }

    fn clear_pad_button_axis(app: &mut App, game_button: GamepadButtonType) {
        app.world.resource_mut::<Axis<GamepadButton>>().set(GamepadButton::new(Gamepad::new(1), game_button), 0.);
    }


    fn pad_axis(app: &mut App, axis: GamepadAxisType) {
        app.world.resource_mut::<Axis<GamepadAxis>>().set(GamepadAxis::new(Gamepad::new(1), axis), 1.0);
    }


    fn clear_pad_axis(app: &mut App, axis: GamepadAxisType) {
        app.world.resource_mut::<Axis<GamepadAxis>>().set(GamepadAxis::new(Gamepad::new(1), axis), 0.);
    }


    fn read(
        mut commands: Commands,
        mut er: EventReader<MyEvent>,
    ) {
        for _ in er.iter() {
            commands.spawn(EventSent);
        }
    }

    fn new_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, read);
        app.add_event::<MyEvent>();
        app.add_systems(Update, (
            input_system::<MyEvent>,
            start_input_system::<MyEvent>,
        ).chain());
        app.init_resource::<Gamepads>();
        app.init_resource::<Input<GamepadButton>>();
        app.init_resource::<Input<GamepadAxis>>();
        app.init_resource::<Axis<GamepadButton>>();
        app.init_resource::<Axis<GamepadAxis>>();
        app.init_resource::<Input<KeyCode>>();

        app
    }
}