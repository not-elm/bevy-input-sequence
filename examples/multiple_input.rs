use std::time::Duration;

use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Event, EventReader, GamepadButtonType, KeyCode};

use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::prelude::{Act, InputSequence, Timeout};


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
        Timeout::from_duration(Duration::from_secs(5)),
        &[
            Act::Key(KeyCode::W) | Act::PadButton(GamepadButtonType::North),
            Act::Key(KeyCode::D) | Act::PadButton(GamepadButtonType::East),
            Act::Key(KeyCode::S) | Act::PadButton(GamepadButtonType::South),
            Act::Key(KeyCode::A) | Act::PadButton(GamepadButtonType::West)
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