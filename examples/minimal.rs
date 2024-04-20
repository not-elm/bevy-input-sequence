use bevy::prelude::*;
use bevy_input_sequence::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputSequencePlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.add(KeySequence::new(say_hi, keyseq! { H I }));
}

fn say_hi() {
    info!("hi");
}
