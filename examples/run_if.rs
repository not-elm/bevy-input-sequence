use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

#[derive(Message, Clone, Debug)]
struct MyEvent;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    Menu,
    #[default]
    Game,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MySet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(InputSequencePlugin::empty().run_in_set(Update, MySet))
        .add_message::<MyEvent>()
        .configure_sets(Update, MySet.run_if(in_state(AppState::Game)))
        .add_systems(Startup, setup)
        .add_systems(Update, listen_for_myevent)
        .add_systems(Update, listen_for_escape_key)
        .run();
}

fn setup(mut commands: Commands) {
    commands.queue(
        KeySequence::new(
            action::send_event(MyEvent).only_if(in_state(AppState::Game)),
            keyseq! { Space },
        )
        .time_limit(Duration::from_secs(1)),
    );
    println!("Press Space to emit event in game mode.");
    println!("Press Escape to switch between Game and Menu mode; currently in Game mode.");
}

fn listen_for_escape_key(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        let new_state = match state.get() {
            AppState::Menu => AppState::Game,
            AppState::Game => AppState::Menu,
        };
        println!("Going to state {:?}.", new_state);
        next_state.set(new_state);
    }
}

fn listen_for_myevent(mut er: MessageReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted.");
    }
}
