use std::time::Duration;

use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, Gamepad, GamepadButtonType};
use bevy::DefaultPlugins;

use bevy_input_sequence::InputSequencePlugin;
use bevy_input_sequence::{action, keyseq, ButtonSequence, KeySequence};

#[derive(Event, Clone, Debug)]
struct MyEvent(u8, Option<Gamepad>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputSequencePlugin::default())
        .add_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, input_sequence_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.add(
        KeySequence::new(action::send_event(MyEvent(1, None)), keyseq!(W D S A))
            .time_limit(Duration::from_secs(5)),
    );

    commands.add(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(2, Some(gamepad))),
            [
                GamepadButtonType::North,
                GamepadButtonType::East,
                GamepadButtonType::South,
                GamepadButtonType::West,
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    commands.add(
        KeySequence::new(action::send_event(MyEvent(3, None)), keyseq!(W A S D))
            .time_limit(Duration::from_secs(5)),
    );

    commands.add(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(4, Some(gamepad))),
            [
                GamepadButtonType::North,
                GamepadButtonType::West,
                GamepadButtonType::South,
                GamepadButtonType::East,
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    println!("Press W D S A or north east south west to emit event 1 and 2.");
    println!("Press W A S D or north west south east to emit event 3 and 4.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!(
            "{e:?} emitted {}",
            e.1.map(|x| format!("from gamepad id {}", x.id))
               .unwrap_or("not from gamepad".into())
        );
    }
}
