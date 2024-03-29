use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, Gamepad, GamepadButtonType};
use bevy::{DefaultPlugins, utils::Duration};

use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::{ButtonSequence, GamepadEvent};

#[derive(Event, Clone, Debug)]
struct MyEvent(usize, Gamepad);

impl GamepadEvent for MyEvent {
    fn gamepad(&self) -> Option<Gamepad> {
        Some(self.1)
    }

    fn set_gamepad(&mut self, gamepad: Gamepad) {
        self.1 = gamepad;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_button_sequence_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, input_sequence_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(ButtonSequence::new(
        MyEvent(0, Gamepad { id: 999 }),
        [
            GamepadButtonType::North,
            GamepadButtonType::East,
            GamepadButtonType::South,
            GamepadButtonType::West,
        ],
    ).time_limit(Duration::from_secs(1)));

    commands.spawn(ButtonSequence::new(
        MyEvent(1, Gamepad { id: 999 }),
        [
            GamepadButtonType::North,
            GamepadButtonType::West,
            GamepadButtonType::South,
            GamepadButtonType::East,
        ],
    ).time_limit(Duration::from_secs(1)));

    println!("Press north, east, south, west to emit MyEvent 0.");
    println!("Press north, west, south, east to emit MyEvent 1.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted from gamepad {}", e.gamepad()
                 .map(|x| format!("id {}", x.id)).unwrap_or("UNKNOWN".into()));
    }
}
