use bevy::prelude::*;
use bevy_input_sequence::prelude::Act::*;
use bevy_input_sequence::prelude::*;
use std::time::Duration;

#[derive(Clone, Debug)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Event, Clone, Debug)]
struct MyEvent(Direction);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_input_sequence_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, event_listener)
        .run();
}

#[rustfmt::skip]
fn setup(mut commands: Commands) {
    // Specify key codes directly.
    commands.spawn(
        InputSequence::new(
            MyEvent(Direction::Clockwise),
            [KeyCode::W,
             KeyCode::D,
             KeyCode::S,
             KeyCode::A],
        )
        .time_limit(Duration::from_secs(1)),
    );

    // Use keyseq! macro.
    commands.spawn(
        InputSequence::new(
            MyEvent(Direction::CounterClockwise),
            keyseq!(W A S D),
        )
        .time_limit(Duration::from_secs(1)),
    );
    println!("Press W D S A to emit clockwise event.");
    println!("Press W A S D to emit counter clockwise event.");
}

fn event_listener(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted.");
    }
}
