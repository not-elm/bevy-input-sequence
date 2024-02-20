# bevy-input-sequence

This crate recognizes input sequences and sends events.

# Use cases

* Hotkeys
* Cheat codes
* Developer UI

# Installation

``` sh
cargo install bevy-input-sequence
```

# Usage

## Import symbols

```compile
use bevy::prelude::*;
use bevy_input_sequence::*;
```

## Define an event

```ignore
#[derive(Event, Clone, Debug)]
struct MyEvent;
```

## Add event as an input_sequence

```ignore
fn main() {
    App::new()
        .add_input_sequence_event::<MyEvent>()
        .run()
}
```

## Add a key sequence component

So long as one component is present, it will fire one event when the input
sequence is entered. This crate re-exports the `keyseq!` macro for bevy from the [keyseq](https://crates.io/crates/keyseq) crate.

```ignore
fn setup(mut commands: Commands) {
    commands.spawn(
        InputSequence::new(MyEvent, keyseq! { alt-X M })
    );
}
```

# Examples

This is the `keycode` example. It will recognize the key sequences `W D S A` and
`W A S D` and fire an event. 

``` sh
cargo run --example keycode
```

```compile
use bevy::prelude::*;
use bevy_input_sequence::*;
use std::time::Duration;

#[derive(Clone, Debug)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

#[derive(Event, Clone, Debug)]
struct MyEvent(Direction);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_input_sequence_event::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, event_listener)
        .run();
}

fn setup(mut commands: Commands) {
    // Use keyseq! macro.
    commands.spawn(
        InputSequence::new(
            MyEvent(Direction::CounterClockwise),
            keyseq! { W A S D },
        )
        .time_limit(Duration::from_secs(1)),
    );

    // Specify key codes directly.
    commands.spawn(
        InputSequence::new(
            MyEvent(Direction::Clockwise),
            [KeyCode::W,
             KeyCode::D,
             KeyCode::S,
             KeyCode::A],
        )
    );

    println!("Press W D S A to emit clockwise event.");
    println!("Press W A S D to emit counter clockwise event.");
}

fn event_listener(mut er: EventReader<MyEvent>) {
    for e in er.read() {
        println!("{e:?} emitted.");
    }
}
```

See [`here`](./examples/) for more examples.

# Advanced Usage

## Add an event with a condition

Some key sequences you may only what to fire in particular modes. You can supply
a condition that will only run if it's met. This works nicely with bevy `States`
for example.

```ignore
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_input_sequence_event_run_if::<MyEvent, _>(in_state(AppState::Menu))
        .run()
}
```

See the "run_if" example for more details.

## Add a input sequence component with a time limit

Input sequences can have time limits. Sequences must be completed within the
time limit in order to fire the event.

```ignore
fn setup(mut commands: Commands) {
    commands.spawn(
        InputSequence::new(MyEvent, keyseq! { alt-X M })
            .time_limit(Duration::from_secs(1)),
    );
}
```

# Compatibility

| bevy-input-sequence | bevy |
| ------------------- | ---- |
| 0.1.0               | 0.11 |
| 0.2.0               | 0.12 |
| N/A                 | 0.13 |

# License

This crate is licensed under the MIT License or the Apache License 2.0.
