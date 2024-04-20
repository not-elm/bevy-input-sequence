use bevy::{
    ecs::{
        world::World,
        event::{Event, EventWriter},
        system::{System, In, IntoSystem, ReadOnlySystem, SystemId},
        schedule::Condition,
    },
    input::gamepad::Gamepad,
};

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
/// enum AppState {
///     Menu,
///     Game,
/// }
/// #[derive(Event, Clone, Debug)]
/// struct MyEvent;
/// KeySequence::new(
///    action::send_event_if(MyEvent, in_state(AppState::Game)),
///    keyseq! { Space });
/// ```
pub fn send_event_if<E: Event + Clone, M>(event: E, condition: impl Condition<M>) -> impl FnMut(&mut World) {
    let mut condition_system = Some(IntoSystem::into_system(condition));
    let mut system_id: Option<SystemId<(), bool>> = None;
    move |world: &mut World| {
        if system_id.is_none() {
            system_id = Some(world.register_system(condition_system.take().expect("No condition system")));
        }
        if world.run_system(system_id.expect("Condition not registered")).expect("Condition run failed") {
            world.send_event(event.clone());
        }
    }
}

pub fn send_gamepad_event<E: Event, F: FnMut(Gamepad) -> E>(mut f: F) -> impl FnMut(In<Gamepad>, EventWriter<E>) {
    move |In(gamepad), mut writer: EventWriter<E>| {
        writer.send(f(gamepad));
    }
}
