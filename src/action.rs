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

pub fn send_event_if<E: Event + Clone, M>(event: E, condition: impl Condition<M>) -> impl FnMut(&mut World) {
    let mut condition_system = Some(IntoSystem::into_system(condition));
    let mut system_id: Option<SystemId<(), bool>> = None;
    move |world: &mut World| {
        if system_id.is_none() {
            system_id = Some(world.register_system(condition_system.take().unwrap()));
        }
        if world.run_system(system_id.unwrap()).unwrap() {
            world.send_event(event.clone());
        }
    }
}

pub fn send_gamepad_event<E: Event, F: FnMut(Gamepad) -> E>(mut f: F) -> impl FnMut(In<Gamepad>, EventWriter<E>) {
    move |In(gamepad), mut writer: EventWriter<E>| {
        writer.send(f(gamepad));
    }
}
