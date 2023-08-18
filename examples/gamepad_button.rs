use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Event, EventReader, GamepadButtonType};

use bevy_secret_command::AppSecretCommandEx;
use bevy_secret_command::prelude::{InputSequence, Timeout};

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
    commands.spawn(InputSequence::from_pad_buttons(
        MyEvent,
        Timeout::default(),
        &[
            GamepadButtonType::North,
            GamepadButtonType::East,
            GamepadButtonType::South,
            GamepadButtonType::West,
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