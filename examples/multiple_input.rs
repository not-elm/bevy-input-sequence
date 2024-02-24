use std::time::Duration;

use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, GamepadButtonType, KeyCode};
use bevy::DefaultPlugins;

use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::{Act, InputSequence};

#[derive(Event, Clone, Debug)]
struct MyEvent;

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
            MyEvent,
            [
                Act::from(KeyCode::W) | Act::from(GamepadButtonType::North),
                Act::from(KeyCode::D) | Act::from(GamepadButtonType::East),
                Act::from(KeyCode::S) | Act::from(GamepadButtonType::South),
                Act::from(KeyCode::A) | Act::from(GamepadButtonType::West),
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    commands.spawn(
        InputSequence::new(
            MyEvent,
            [
                Act::from(KeyCode::W) | Act::from(KeyCode::I),
                Act::from(KeyCode::A) | Act::from(KeyCode::J),
                Act::from(KeyCode::D) | Act::from(KeyCode::K),
                Act::from(KeyCode::S) | Act::from(KeyCode::L),
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );
    println!("Press W D S A or north east south west to emit event.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted ");
    }
}
