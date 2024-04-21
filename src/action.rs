//! Common actions to do on key sequence matches
use bevy::ecs::{
    event::{Event, EventWriter},
    system::In,
};

/// Send this event.
///
/// ```
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

/// Sends an event with input, .e.g, [ButtonSequence] provides a [Gamepad] identifier.
pub fn send_event_with_input<E: Event, Input: 'static, F: FnMut(Input) -> E>(
    mut f: F,
) -> impl FnMut(In<Input>, EventWriter<E>) {
    move |In(x), mut writer: EventWriter<E>| {
        writer.send(f(x));
    }
}
