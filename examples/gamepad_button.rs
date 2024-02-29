use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, Gamepad, GamepadButtonType};
use bevy::DefaultPlugins;

use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::{ButtonSequence, GamepadEvent};

#[derive(Event, Clone, Debug)]
struct MyEvent(Gamepad);

impl GamepadEvent for MyEvent {
    fn gamepad(&self) -> Option<Gamepad> {
        Some(self.0)
    }

    fn set_gamepad(&mut self, gamepad: Gamepad) {
        self.0 = gamepad;
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
        MyEvent(Gamepad { id: 999 }),
        [
            GamepadButtonType::North,
            GamepadButtonType::East,
            GamepadButtonType::South,
            GamepadButtonType::West,
        ],
    ));
    println!("Press north, east, south, west to emit MyEvent.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted from gamepad {:?}", e.gamepad());
    }
}
