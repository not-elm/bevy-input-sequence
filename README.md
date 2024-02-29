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

```rust ignore
use bevy::prelude::*;
use bevy_input_sequence::*;
```

## Define an event

```rust ignore
#[derive(Event, Clone, Debug)]
struct MyEvent;
```

## Add event as an key sequence

```rust ignore
fn main() {
    App::new()
        .add_key_sequence_event::<MyEvent>()
        .run()
}
```

## Add a key sequence component

So long as one component is present, it will fire one event when the input
sequence is entered. This crate re-exports the `keyseq!` macro for bevy from the [keyseq](https://crates.io/crates/keyseq) crate.

```rust ignore
fn setup(mut commands: Commands) {
    commands.spawn(
        KeySequence::new(MyEvent, keyseq! { alt-X M })
    );
}
```

# Examples

## keycode

The `keycode` example recognizes the key sequences `W D S A` and `W A S D` and
fires a distinct event.

``` sh
cargo run --example keycode
```

## keymod

The `keymod` example recognizes `ctrl-W ctrl-D ctrl-S ctrl-A` and fires an event.

``` sh
cargo run --example keymod
```

## gamepad_button

The `gamepad_button` example recognizes gamepad buttons `North East South West`
or `Y B A X` on an Xbox controller and fires an event.

``` sh
cargo run --example gamepad_button
```

## multiple_input

The `multiple_input` example recognizes gamepad buttons `North East South West`,
or `Y B A X` on an Xbox controller, or `W D S A` on a keyboard and fires an
event.

``` sh
cargo run --example multiple_input
```

Note: Either `W D S A` will be recognized from the keyboard, or `Y B A X` will
be recognized from the controller. But a mixed sequence like `W D A X` will not
currently be recognized. If this should be done and how exactly one should do it
are under consideration. Please open an issue or PR if you have thoughts on this.

## run_if

The `run_if` example recognizes `Space` and fires an event if it's in game mode.
The `Escape` key toggles the app betwee n menu and game mode.

``` sh
cargo run --example run_if
```

# Advanced Usage

## Fire event on gamepad button sequence

Define an event

```rust ignore
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

```

Add event as a button sequence.

```rust ignore
fn main() {
    App::new()
        .add_button_sequence_event::<MyEvent>()
        .run()
}
```

Add a button sequence component.

```rust ignore
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
```

## Add an event with a condition

Some key sequences you may only what to fire in particular modes. You can supply
a condition that will only run if it's met. This works nicely with bevy `States`
for example.

```rust ignore
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_key_sequence_event_run_if::<MyEvent, _>(in_state(AppState::Menu))
        .run()
}
```

See the "run_if" example for more details.

## Add a input sequence component with a time limit

Input sequences can have time limits. Sequences must be completed within the
time limit in order to fire the event.

```rust ignore
fn setup(mut commands: Commands) {
    commands.spawn(
        KeySequence::new(MyEvent, keyseq! { alt-X M })
            .time_limit(Duration::from_secs(1)),
    );
}
```

# Compatibility

| bevy-input-sequence | bevy |
| ------------------- | ---- |
| 0.1                 | 0.11 |
| 0.2                 | 0.12 |
| 0.3                 | 0.13 |

# License

This crate is licensed under the MIT License or the Apache License 2.0.
