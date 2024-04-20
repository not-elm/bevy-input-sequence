use crate::{
    time_limit::TimeLimit,
    KeyChord,
};
use bevy::{
    ecs::{
        entity::Entity,
        system::{BoxedSystem, IntoSystem, SystemId},
        world::World,
    },
    input::gamepad::Gamepad,
    prelude::{Component, Event, GamepadButtonType},
    reflect::Reflect,
};

/// An input sequence is a series of acts [A] that fires an event when matched
/// with inputs within the given time limit.
#[derive(Component, Reflect, Clone)]
#[reflect(from_reflect = false)]
pub struct InputSequence<Act, In> {
    /// Event emitted
    #[reflect(ignore)]
    pub system_id: SystemId<In>,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<Act>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
}

pub struct InputSequenceBuilder<A, In> {
    pub system: BoxedSystem<In>,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<A>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
}

impl<A, In> InputSequenceBuilder<A, In>
where
    In: 'static,
{
    /// Create new input sequence. Not operant until added to an entity.
    pub fn new<S, P>(system: S) -> Self
    where
        S: IntoSystem<In, (), P> + 'static,
    {
        InputSequenceBuilder {
            acts: Vec::new(),
            system: Box::new(IntoSystem::into_system(system)),
            time_limit: None,
        }
    }

    /// Specify a time limit from the start of the first matching input.
    pub fn time_limit(mut self, time_limit: impl Into<TimeLimit>) -> Self {
        self.time_limit = Some(time_limit.into());
        self
    }

    pub fn build(mut self, world: &mut World) -> InputSequence<A, In> {
        InputSequence {
            system_id: world.register_boxed_system::<In, ()>(self.system),
            acts: self.acts,
            time_limit: self.time_limit,
        }
    }
}

impl<A, In> bevy::ecs::system::Command for InputSequenceBuilder<A, In>
where
    A: Send + Sync + 'static,
    In: Send + Sync + 'static,
{
    fn apply(self, world: &mut World) {
        let act = self.build(world);
        world.spawn(act);
    }
}

impl<A, In> bevy::ecs::system::EntityCommand for InputSequenceBuilder<A, In>
where
    A: Send + Sync + 'static,
    In: Send + Sync + 'static,
{
    fn apply(self, id: Entity, world: &mut World) {
        let act = self.build(world);
        let mut entity = world.get_entity_mut(id).unwrap();
        entity.insert(act);
    }
}

impl<A, In> InputSequence<A, In>
where
    In: 'static,
{
    /// Create new input sequence. Not operant until added to an entity.
    #[inline(always)]
    pub fn new<T, S, P>(system: S, acts: impl IntoIterator<Item = T>) -> InputSequenceBuilder<A, In>
    where
        A: From<T>,
        S: IntoSystem<In, (), P> + 'static,
    {
        let mut builder = InputSequenceBuilder::new(system);
        builder.acts = Vec::from_iter(acts.into_iter().map(A::from));
        builder
    }
}

/// Represents a key sequence.
pub type KeySequence = InputSequence<KeyChord, ()>;

/// Represents a gamepad button sequence.
pub type ButtonSequence = InputSequence<GamepadButtonType, Gamepad>;
