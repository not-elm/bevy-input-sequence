use std::time::Duration;

use bevy::prelude::*;

use bevy_input_sequence::prelude::*;

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
        Timeout::from_duration(Duration::from_secs(1)),
        [
            // KeyCode::ControlLeft,
            KeyCode::W,
            KeyCode::D,
            KeyCode::S,
            KeyCode::A,
        ],
    ));

    commands.spawn(InputSequence::new(
        MyEvent,
        Timeout::from_duration(Duration::from_secs(1)),
        [KeyCode::Z],
    ));
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} Coming ");
    }
}
