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
    commands.spawn(
        InputSequence::new(
            MyEvent,
            // [
            keyseq!(W D S A),
            // keyseq!(ctrl-W D S A),
            // (Modifiers::Control, KeyCode::W),
                                   // (Modifiers::empty(), KeyCode::D),
                                   // (Modifiers::empty(), KeyCode::S),
                                   // (Modifiers::empty(), KeyCode::A),
                                   // ],
        )
        .time_limit(Duration::from_secs(1)),
    );
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted ");
    }
}
