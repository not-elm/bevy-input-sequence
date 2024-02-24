use std::time::Duration;

use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, GamepadButtonType, KeyCode};
use bevy::DefaultPlugins;

use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::{Act, InputSequence};

#[derive(Event, Clone, Debug)]
struct MyEvent(u8);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_input_sequence_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, input_sequence_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        InputSequence::new(
            MyEvent(1),
            [
                Act::from(KeyCode::KeyW) | Act::from(GamepadButtonType::North),
                Act::from(KeyCode::KeyD) | Act::from(GamepadButtonType::East),
                Act::from(KeyCode::KeyS) | Act::from(GamepadButtonType::South),
                Act::from(KeyCode::KeyA) | Act::from(GamepadButtonType::West),
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    // BUG: W A S D does not work!
    commands.spawn(
        InputSequence::new(
            MyEvent(2),
            [
                Act::from(KeyCode::KeyW) | Act::from(KeyCode::KeyI),
                Act::from(KeyCode::KeyA) | Act::from(KeyCode::KeyJ),
                Act::from(KeyCode::KeyS) | Act::from(KeyCode::KeyK),
                Act::from(KeyCode::KeyD) | Act::from(KeyCode::KeyL),
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );
    println!("Press W D S A or north east south west to emit event 1.");
    println!("Press W A S D or I J K L to emit event 2.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted ");
    }
}
