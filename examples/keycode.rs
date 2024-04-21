use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

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
        .add_plugins(InputSequencePlugin::default())
        .add_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, event_listener)
        .run();
}

#[rustfmt::skip]
fn setup(mut commands: Commands) {
    // Specify key codes directly.
    commands.add(
        KeySequence::new(
            action::send_event(MyEvent(Direction::Clockwise)),
            [KeyCode::KeyW,
             KeyCode::KeyD,
             KeyCode::KeyS,
             KeyCode::KeyA],
        )
        .time_limit(Duration::from_secs(1)),
    );

    // Use keyseq! macro.
    commands.add(
        KeySequence::new(
            action::send_event(MyEvent(Direction::CounterClockwise)),
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
