# bevy-input-sequence

This crate provides recognizes input sequences and send events.

## Use cases

* Hotkeys
* Cheat codes
* Developer UI

## Examples

This is the `keycode` example. It will recognize the key sequences `W D S A` and
`W A S D` and fire an event. 

``` sh
cargo run --example keycode
```

```compile
use std::time::Duration;
use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

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
    println!("Recognizes W D S A and W A S D key sequences.");
    commands.spawn(InputSequence::new(
        MyEvent(Direction::Clockwise),
        [
            KeyCode::W,
            KeyCode::D,
            KeyCode::S,
            KeyCode::A
        ],
    ).timeout(Duration::from_secs(1)));

    commands.spawn(InputSequence::new(
        MyEvent(Direction::CounterClockwise),
        keyseq!(W A S D),
    ).timeout(Duration::from_secs(1)));
}

fn event_listener(
    mut er: EventReader<MyEvent>
) {
    for e in er.read() {
        println!("{e:?} Coming ");
    }
}
```

See [`here`](./examples/) for more examples.
