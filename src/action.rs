//! Common actions to do on key sequence matches
use bevy::ecs::{
    prelude::{Commands, Event, Message, MessageWriter},
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
pub fn send_event<E: Message + Clone>(event: E) -> impl FnMut(MessageWriter<E>) {
    move |mut writer: MessageWriter<E>| {
        writer.write(event.clone());
    }
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

/// Sends an event with input, .e.g,
/// [ButtonSequence](crate::input_sequence::ButtonSequence) provides a
/// [Gamepad](bevy::input::gamepad::Gamepad) identifier.
pub fn send_event_with_input<E: Message, Input: 'static, F: FnMut(Input) -> E>(
    mut f: F,
) -> impl FnMut(In<Input>, MessageWriter<E>) {
    move |In(x), mut writer: MessageWriter<E>| {
        writer.write(f(x));
    }
}
