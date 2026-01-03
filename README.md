# bevy-input-sequence

Recognizes and acts on input sequences from the keyboard or a gamepad.

# Use Cases

* Hotkeys
* Cheat codes
* Developer UI

# Installation

``` sh
cargo install bevy-input-sequence
```

# Code Examples

Here are some code snippets. These also run as doctests so they do a few things
differently than a regular runnable example:

- Instead of `DefaultPlugins` they use `MinimalPlugins`.
- Instead of `app.run()` they call `app.update()`.

The next section describes the runnable examples that come with the crate.

## Run a System on a Key Sequence

Run a system whenever the user presses the key sequence `H I` or "hi" within a
time limit.

```rust
use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(InputSequencePlugin::default())
        .add_systems(Startup, setup)
        .update(); // Normally you'd run it here.
}

fn setup(mut commands: Commands) {
    // Add key sequence.
    commands.queue(
        KeySequence::new(say_hello, 
                         keyseq! { H I })
        .time_limit(Duration::from_secs(2))
    );
}

fn say_hello() {
    info!("hello");
}
```

## Send an Event on Key Sequence

Originally `bevy-input-sequence` always sent an event. You can still do that
with `action::write_message()`.

```rust
use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

/// Define an event.
#[derive(Message, Clone, Debug, Default)]
struct MyEvent;

/// Add event as an key sequence.
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(InputSequencePlugin::default())
        .add_message::<MyEvent>()
        .add_systems(Startup, setup)
        .update(); // Normally you'd run it here.
}

fn setup(mut commands: Commands) {
    commands.queue(
        KeySequence::new(action::write_message(MyEvent), 
                         keyseq! { Ctrl-E Alt-L Shift-M })
    );
}

fn check_events(mut events: MessageReader<MyEvent>) {
    for event in events.read() {
        info!("got event {event:?}");
    }
}
```

## Send an Event on Gamepad Button Sequence

Gamepads have something that keyboards don't: identity problems. Which player
hit the button sequence may be important to know. So the systems it accepts 
take an input of `Entity`.

```rust
use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

/// Define an event.
#[derive(Message, Clone, Debug)]
struct MyEvent(Entity);

/// Add event as an key sequence.
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(InputSequencePlugin::default())
        .add_message::<MyEvent>()
        .add_systems(Startup, setup)
        .update(); // Normally you'd run it here.
}

fn setup(mut commands: Commands) {
    commands.queue(
        ButtonSequence::new(action::write_message_with_input(|gamepad| MyEvent(gamepad)), 
            [GamepadButton::North,
             GamepadButton::East,
             GamepadButton::South,
             GamepadButton::West])
    );
}

fn check_events(mut events: MessageReader<MyEvent>) {
    for event in events.read() {
        info!("got event {event:?}");
    }
}
```

## Trigger an Event on Key Sequence

You can also trigger an event with `action::trigger()` or `action::trigger_targets()`.

```rust
use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

/// Define an event.
#[derive(Event, Clone, Debug, Default)]
struct MyEvent;

/// Add event as an key sequence.
fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(InputSequencePlugin::default())
        .add_systems(Startup, setup)
        .add_observer(check_trigger)
        .update(); // Normally you'd run it here.
}

fn setup(mut commands: Commands) {
    commands.queue(
        KeySequence::new(action::trigger(MyEvent), 
                         keyseq! { Ctrl-E Alt-L Super-M })
    );
}

fn check_trigger(mut trigger: On<MyEvent>) {
    info!("got event {:?}", trigger.event());
}
```

## KeySequence Creation Patterns

`KeySequence::new` now returns `KeySequenceBuilder`, which implements `Command`.
Therefore, you need to call `Commands::queue` instead of `Commands::spawn`.

```rust
use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

#[derive(Message, Clone, Debug, Default)]
struct MyEvent;

fn create_key_sequence(mut commands: Commands) {
    commands.queue(KeySequence::new(
        action::write_message(bevy::app::AppExit::default()), 
        keyseq! { Ctrl-E L M }
    ));
}

fn create_key_sequence_and_add_it_to_an_entity(mut commands: Commands) {
    let id = commands.spawn_empty().id();
    commands.entity(id).queue(KeySequence::new(
        action::write_message(MyEvent), 
        keyseq! { Ctrl-E L M }
    ));
    // OR
    commands.spawn_empty().queue(KeySequence::new(
        action::write_message(MyEvent), 
        keyseq! { Ctrl-E L M }
    ));
}
```

### Advanced Creation

The `KeySequenceBuilder` requires a `&mut World` to build it. You can build it
yourself like so:

```rust
use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

fn create_key_sequence_within_command(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        let builder = KeySequence::new(
            move || { info!("got it"); },
            keyseq! { Ctrl-E L M }
        );
        let key_sequence: KeySequence = builder.build(world);
        // And then put it somewhere? It ought to go as a component.
    });
}
```

# Runnable Examples

## keycode

The `keycode` example recognizes the key sequences `W D S A` and `W A S D` and
fires a distinct event.

``` sh
cargo run --example keycode
```

## keymod

The `keymod` example recognizes `Ctrl-W Ctrl-D Ctrl-S Ctrl-A` and fires an event.

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

## only_if

The `only_if` example recognizes `Space` and fires an event if it's in game
mode. The `Escape` key toggles the app between menu and game mode. It does this
by only sending the `Space` event if it's in game mode.

``` sh
cargo run --example only_if
```

## run_if

The `run_if` has the same behavior as `only_if` but achieves it differently. It
places the `InputSequencePlugin` systems in a system set that is configured to
only run in game mode. Because of this the `Escape` key which toggles between
game and menu mode cannot be a `KeySequence`.

``` sh
cargo run --example run_if
```

# Compatibility

| bevy-input-sequence | bevy |
|---------------------|------|
| 0.9                 | 0.17 |
| 0.8                 | 0.16 |
| 0.7                 | 0.15 |
| 0.5 ~ 0.6           | 0.14 |
| 0.3 ~ 0.4           | 0.13 |
| 0.2                 | 0.12 |
| 0.1                 | 0.11 |

# License

This crate is licensed under the MIT License or the Apache License 2.0.
