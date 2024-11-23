use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

#[derive(Event, Clone, Debug)]
struct MyEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MyEvent>()
        .add_plugins(InputSequencePlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, input_sequence_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.add(
        KeySequence::new(
            action::send_event(MyEvent),
            keyseq!(Ctrl-W Ctrl-D Ctrl-S Ctrl-A),
        )
        .time_limit(Duration::from_secs(1)),
    );
    println!("Press Ctrl-W Ctrl-D Ctrl-S Ctrl-A to emit event.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted.");
    }
}
