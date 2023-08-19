use std::time::Duration;

use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Event, EventReader, KeyCode};

use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::prelude::{InputSequence, Timeout};

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
    commands.spawn(InputSequence::from_keycodes(
        MyEvent,
        Timeout::from_duration(Duration::from_secs(1)),
        &[
            KeyCode::W,
            KeyCode::D,
            KeyCode::S,
            KeyCode::A
        ],
    ));
}


fn input_sequence_event_system(
    mut er: EventReader<MyEvent>
) {
    for e in er.iter() {
        println!("{e:?} Coming ");
    }
}