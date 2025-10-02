use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

#[derive(Message, Clone, Debug)]
struct MyEvent;

#[derive(Message, Clone, Debug)]
struct GlobalEvent;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    Menu,
    #[default]
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(InputSequencePlugin::default())
        .add_message::<GlobalEvent>()
        .add_message::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, listen_for_myevent)
        .add_systems(Update, listen_for_global_event)
        .run();
}

fn setup(mut commands: Commands) {
    commands.queue(KeySequence::new(
        action::send_event(GlobalEvent),
        keyseq! { Escape },
    ));
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

fn listen_for_global_event(
    mut er: MessageReader<GlobalEvent>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in er.read() {
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
