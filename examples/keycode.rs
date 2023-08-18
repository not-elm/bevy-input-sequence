use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Event, EventReader, KeyCode};

use bevy_secret_command::AppSecretCommandEx;
use bevy_secret_command::prelude::{InputSequence, Timeout};

#[derive(Event, Clone, Debug)]
enum KonamiCommandEvent {
    Short,
    Long,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_secret_command_event::<KonamiCommandEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, secret_event_system)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(InputSequence::from_keycodes(
        KonamiCommandEvent::Short,
        Timeout::default(),
        &[
            KeyCode::Up,
            KeyCode::Up,
            KeyCode::Down
        ],
    ));

    commands.spawn(InputSequence::from_keycodes(
        KonamiCommandEvent::Long,
        Timeout::default(),
        &[
            KeyCode::Up,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::B,
            KeyCode::A
        ],
    ));
}


fn secret_event_system(
    mut er: EventReader<KonamiCommandEvent>
) {
    for e in er.iter() {
        println!("KonamiCommandEvent::{e:?} Coming ");
    }
}