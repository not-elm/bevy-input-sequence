use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Event, EventReader, GamepadAxisType, GamepadButtonType, KeyCode};

use bevy_secret_command::AppSecretCommandEx;
use bevy_secret_command::prelude::{Entry, InputSequence, Timeout};

#[derive(Event, Clone, Debug)]
struct MyEvent;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_secret_command_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, secret_event_system)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(InputSequence::new(
        MyEvent,
        Timeout::default(),
        &[
            Entry::Key(KeyCode::A),
            Entry::PadButton(GamepadButtonType::East) | Entry::Key(KeyCode::B),
            Entry::PadAxis(GamepadAxisType::LeftStickY),
            Entry::PadButtonAxis(GamepadButtonType::LeftTrigger)
        ],
    ));
}


fn secret_event_system(
    mut er: EventReader<MyEvent>
) {
    for e in er.iter() {
        println!("KonamiCommandEvent::{e:?} Coming ");
    }
}