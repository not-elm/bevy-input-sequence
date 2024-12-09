use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

#[derive(Event, Clone, Debug)]
#[allow(dead_code)]
struct MyEvent(u8, Option<Entity>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputSequencePlugin::default())
        .add_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, input_sequence_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.queue(
        KeySequence::new(action::send_event(MyEvent(1, None)), keyseq! { W D S A })
            .time_limit(Duration::from_secs(5)),
    );

    commands.queue(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(2, Some(gamepad))),
            [
                GamepadButton::North,
                GamepadButton::East,
                GamepadButton::South,
                GamepadButton::West,
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    commands.queue(
        KeySequence::new(action::send_event(MyEvent(3, None)), keyseq! { W A S D })
            .time_limit(Duration::from_secs(5)),
    );

    commands.queue(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(4, Some(gamepad))),
            [
                GamepadButton::North,
                GamepadButton::West,
                GamepadButton::South,
                GamepadButton::East,
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    println!("Press W D S A or north east south west to emit event 1 and 2.");
    println!("Press W A S D or north west south east to emit event 3 and 4.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!(
            "{e:?} emitted {}",
            e.1.map(|x| format!("from gamepad id {}", x))
                .unwrap_or("not from gamepad".into())
        );
    }
}
