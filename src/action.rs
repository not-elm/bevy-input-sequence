//! Common actions to do on key sequence matches
use bevy::ecs::{
    event::{Event, EventWriter},
    observer::TriggerTargets,
    prelude::Commands,
    system::In,
};

/// Send this event.
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_input_sequence::prelude::*;
///
/// #[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
/// enum AppState { Menu, Game }
/// #[derive(Event, Clone, Debug)]
/// struct MyEvent;
///
/// KeySequence::new(
///    action::send_event(MyEvent),
///    keyseq! { Space });
/// ```
pub fn send_event<E: Event + Clone>(event: E) -> impl FnMut(EventWriter<E>) {
    move |mut writer: EventWriter<E>| {
        writer.send(event.clone());
    }
}

/// Trigger and event.
pub fn trigger<E: Event + Clone>(event: E) -> impl FnMut(Commands) {
    move |mut commands: Commands| {
        commands.trigger(event.clone());
    }
}

/// Trigger and event with targets.
pub fn trigger_targets<E: Event + Clone, T: TriggerTargets + Clone>(
    event: E,
    targets: T,
) -> impl FnMut(Commands) {
    move |mut commands: Commands| {
        commands.trigger_targets(event.clone(), targets.clone());
    }
}

/// Sends an event with input, .e.g, [ButtonSequence](crate::input_sequence::ButtonSequence) provides a [Gamepad](bevy::input::gamepad::Gamepad) identifier.
pub fn send_event_with_input<E: Event, Input: 'static, F: FnMut(Input) -> E>(
    mut f: F,
) -> impl FnMut(In<Input>, EventWriter<E>) {
    move |In(x), mut writer: EventWriter<E>| {
        writer.send(f(x));
    }
}
