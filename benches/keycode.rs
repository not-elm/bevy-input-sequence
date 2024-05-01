//! Testing the difference between not using sequence! and using.
#![allow(missing_docs)]

use bevy::app::{App, AppExit, Startup};
use bevy::core::FrameCountPlugin;
use bevy::ecs::event::ManualEventReader;
use bevy::input::InputPlugin;
use bevy::prelude::{ButtonInput, Commands, Events, KeyCode};
use bevy::time::TimePlugin;
use criterion::{Criterion, criterion_group, criterion_main};

use bevy_input_sequence::{action, InputSequencePlugin};
use bevy_input_sequence::prelude::KeySequence;

fn keycode(c: &mut Criterion) {
    c.bench_function("keycode", |b| {
        b.iter(|| {
            let mut app = App::new();
            app
                .add_plugins((
                    InputPlugin,
                    TimePlugin,
                    FrameCountPlugin,
                    InputSequencePlugin::empty().run_in(bevy::prelude::First),
                ))
                .add_systems(Startup, |mut commands: Commands| {
                    commands.add(KeySequence::new(
                        action::send_event(AppExit),
                        [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC],
                    ));
                });

            let codes = [KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC];
            let mut i = 0;
            let mut er = ManualEventReader::<AppExit>::default();
            while er.read(app.world.resource::<Events<AppExit>>()).count() == 0 {
                app.world.resource_mut::<ButtonInput<KeyCode>>().press(codes[i % 3]);
                app.update();
                i += 1;
            }
        });
    });
}

criterion_group!(benchmark, keycode);
criterion_main!(benchmark);