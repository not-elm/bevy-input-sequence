//! Common actions to do on key sequence matches
use bevy::ecs::{
    prelude::{Commands, Event, Message, MessageWriter},
    system::In,
};

/// Write a message.
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_input_sequence::prelude::*;
///
/// #[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
/// enum AppState { Menu, Game }
/// #[derive(Message, Clone, Debug, Default)]
/// struct MyEvent;
///
/// KeySequence::new(
///    action::write_message(MyEvent),
///    keyseq! { Space });
/// ```
pub fn write_message<E: Message + Clone>(event: E) -> impl FnMut(MessageWriter<E>) {
    move |mut writer: MessageWriter<E>| {
        writer.write(event.clone());
    }
}

/// Send an event.
#[deprecated(since = "0.9.1", note = "please use `write_message` instead")]
pub fn send_event<E: Message + Clone>(event: E) -> impl FnMut(MessageWriter<E>) {
    write_message(event)
}

/// Trigger an event.
pub fn trigger<'a, E>(event: E) -> impl FnMut(Commands)
where
    E: Event + Clone,
    <E as Event>::Trigger<'a>: Default,
{
    move |mut commands: Commands| {
        commands.trigger(event.clone());
    }
}

/// Write a message with input, .e.g,
/// [ButtonSequence](crate::input_sequence::ButtonSequence) provides a
/// [Gamepad](bevy::input::gamepad::Gamepad) identifier.
pub fn write_message_with_input<E: Message, Input: 'static, F: FnMut(Input) -> E>(
    mut f: F,
) -> impl FnMut(In<Input>, MessageWriter<E>) {
    move |In(x), mut writer: MessageWriter<E>| {
        writer.write(f(x));
    }
}

/// Sends an event with input.
#[deprecated(
    since = "0.9.1",
    note = "please use `write_message_with_input` instead"
)]
pub fn send_event_with_input<E: Message, Input: 'static, F: FnMut(Input) -> E>(
    f: F,
) -> impl FnMut(In<Input>, MessageWriter<E>) {
    write_message_with_input(f)
}
