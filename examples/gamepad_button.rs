use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, GamepadButtonType};
use bevy::DefaultPlugins;

use bevy_input_sequence::InputSequence;
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
            GamepadButtonType::North,
            GamepadButtonType::East,
            GamepadButtonType::South,
            GamepadButtonType::West,
        ],
    ));
    println!("Press north, east, south, west to emit MyEvent.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted ");
    }
}
