use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

#[derive(Event, Clone, Debug)]
struct MyEvent(usize, Gamepad);

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
    commands.add(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(0, gamepad)),
            [
                GamepadButtonType::North,
                GamepadButtonType::East,
                GamepadButtonType::South,
                GamepadButtonType::West,
            ],
        )
        .time_limit(Duration::from_secs(1)),
    );

    commands.add(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(1, gamepad)),
            [
                GamepadButtonType::North,
                GamepadButtonType::West,
                GamepadButtonType::South,
                GamepadButtonType::East,
            ],
        )
        .time_limit(Duration::from_secs(1)),
    );

    println!("Press north, east, south, west to emit MyEvent 0.");
    println!("Press north, west, south, east to emit MyEvent 1.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{:?} emitted from gamepad {:?}", e.0, e.1);
    }
}
