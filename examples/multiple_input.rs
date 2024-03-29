use std::time::Duration;

use bevy::app::{App, Startup, Update};
use bevy::prelude::{Commands, Event, EventReader, Gamepad, GamepadButtonType};
use bevy::DefaultPlugins;

use bevy_input_sequence::AddInputSequenceEvent;
use bevy_input_sequence::{keyseq, ButtonSequence, GamepadEvent, KeySequence};

#[derive(Event, Clone, Debug)]
struct MyEvent(u8, Option<Gamepad>);

impl GamepadEvent for MyEvent {
    fn gamepad(&self) -> Option<Gamepad> {
        self.1
    }

    fn set_gamepad(&mut self, gamepad: Gamepad) {
        self.1 = Some(gamepad);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_key_sequence_event::<MyEvent>()
        .add_button_sequence_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, input_sequence_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        KeySequence::new(MyEvent(1, None), keyseq!(W D S A)).time_limit(Duration::from_secs(5)),
    );

    commands.spawn(
        ButtonSequence::new(
            MyEvent(2, None),
            [
                GamepadButtonType::North,
                GamepadButtonType::East,
                GamepadButtonType::South,
                GamepadButtonType::West,
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    commands.spawn(
        KeySequence::new(MyEvent(3, None), keyseq!(W A S D)).time_limit(Duration::from_secs(5)),
    );

    commands.spawn(
        ButtonSequence::new(
            MyEvent(4, None),
            [
                GamepadButtonType::North,
                GamepadButtonType::West,
                GamepadButtonType::South,
                GamepadButtonType::East,
            ],
        )
        .time_limit(Duration::from_secs(5)),
    );

    println!("Press W D S A or north east south west to emit event 1 and 2.");
    println!("Press W A S D or north west south east to emit event 3 and 4.");
}

fn input_sequence_event_system(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted {}", e.gamepad()
                 .map(|x| format!("from gamepad id {}", x.id)).unwrap_or("not from gamepad".into()));
    }
}
