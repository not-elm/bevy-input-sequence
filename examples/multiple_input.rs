use std::time::Duration;

use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, GamepadButtonType, KeyCode};
use bevy::DefaultPlugins;

use bevy_input_sequence::prelude::{Act, InputSequence};
use bevy_input_sequence::AddInputSequenceEvent;

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
    commands.spawn(InputSequence::new(
        MyEvent,
        [
            Act::Key(KeyCode::W) | Act::PadButton(GamepadButtonType::North),
            Act::Key(KeyCode::D) | Act::PadButton(GamepadButtonType::East),
            Act::Key(KeyCode::S) | Act::PadButton(GamepadButtonType::South),
            Act::Key(KeyCode::A) | Act::PadButton(GamepadButtonType::West),
        ],
    ).timeout(Duration::from_secs(5)));
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} Coming ");
    }
}
