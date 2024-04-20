//! Common actions to do on key sequence matches
use bevy::ecs::{
    event::{Event, EventWriter},
    schedule::Condition,
    system::{In, IntoSystem, SystemId},
    world::World,
};

/// Send this event.
pub fn send_event<E: Event + Clone>(event: E) -> impl FnMut(EventWriter<E>) {
    move |mut writer: EventWriter<E>| {
        writer.send(event.clone());
    }
}

/// Send an event if a condition is met.
///
/// ```
/// use bevy::prelude::*;
/// use bevy_input_sequence::*;
///
/// #[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
/// enum AppState { Menu, Game }
/// #[derive(Event, Clone, Debug)]
/// struct MyEvent;
///
/// KeySequence::new(
///    action::send_event_if(MyEvent, in_state(AppState::Game)),
///    keyseq! { Space });
/// ```
pub fn send_event_if<E: Event + Clone, M>(
    event: E,
    condition: impl Condition<M>,
) -> impl FnMut(&mut World) {
    let mut condition_system = Some(IntoSystem::into_system(condition));
    let mut system_id: Option<SystemId<(), bool>> = None;
    move |world: &mut World| {
        if system_id.is_none() {
            system_id =
                Some(world.register_system(condition_system.take().expect("No condition system")));
        }
        if world
            .run_system(system_id.expect("Condition not registered"))
            .expect("Condition run failed")
        {
            world.send_event(event.clone());
        }
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
