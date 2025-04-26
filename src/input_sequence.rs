//! Input sequences for keys and gamepad buttons
use crate::{cond_system::IntoCondSystem, time_limit::TimeLimit, KeyChord};
use std::{fmt, marker::PhantomData};

use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        prelude::In,
        system::{IntoSystem, System, SystemId, SystemInput},
        world::World,
    },
    input::gamepad::GamepadButton,
    prelude::{ChildOf, EntityWorldMut},
    reflect::Reflect,
};

/// An input sequence is a series of acts that fires an event when matched with
/// inputs within the given time limit.
///
/// InputSequence<KeyChord, ()>
/// InputSequence<GamepadButton, In<Entity>>
#[derive(Component, Reflect)]
#[reflect(from_reflect = false)]
pub struct InputSequence<Act, I: SystemInput + 'static> {
    /// Event emitted
    #[reflect(ignore)]
    pub system_id: SystemId<I>,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<Act>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
}

impl<Act: Clone> Clone for InputSequence<Act, ()> {
    fn clone(&self) -> Self {
        Self {
            system_id: self.system_id,
            acts: self.acts.clone(),
            time_limit: self.time_limit.clone(),
        }
    }
}

impl<Act: Clone> Clone for InputSequence<Act, In<Entity>> {
    fn clone(&self) -> Self {
        Self {
            system_id: self.system_id,
            acts: self.acts.clone(),
            time_limit: self.time_limit.clone(),
        }
    }
}

impl<Act: fmt::Debug, In: SystemInput + Clone> fmt::Debug for InputSequence<Act, In> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        #[derive(Debug)]
        #[allow(dead_code)]
        struct InputSequence<'a, Act> {
            // system_id: SystemId<In>,
            acts: &'a Vec<Act>,
            time_limit: &'a Option<TimeLimit>,
        }

        let Self {
            acts,
            time_limit,
            system_id: _,
        } = self;

        fmt::Debug::fmt(&InputSequence { acts, time_limit }, f)
    }
}

/// An input sequence builder.
pub struct InputSequenceBuilder<Act, S, I> {
    /// The action when to run when sequence matches
    pub system: S,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<Act>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
    input: PhantomData<I>,
}

impl<Act> InputSequenceBuilder<Act, (), ()> {
    /// Create new input sequence. Not operant until added to an entity.
    pub fn new<C, I, M>(system: C) -> InputSequenceBuilder<Act, C::System, I>
    where
        C: IntoCondSystem<I, (), M> + 'static,
        I: SystemInput + Send + Sync + 'static,
    {
        InputSequenceBuilder {
            acts: Vec::new(),
            system: IntoSystem::into_system(system),
            time_limit: None,
            input: PhantomData,
        }
    }
}

impl<Act, S, I> InputSequenceBuilder<Act, S, I>
where
    S: System<Out = ()>,
{
    /// Specify a time limit from the start of the first matching input.
    pub fn time_limit(mut self, time_limit: impl Into<TimeLimit>) -> Self {
        self.time_limit = Some(time_limit.into());
        self
    }

    /// Build the InputSequence. Requires world to register the system.
    pub fn build(self, world: &mut World) -> InputSequence<Act, S::In> {
        InputSequence {
            system_id: world.register_system(self.system),
            acts: self.acts,
            time_limit: self.time_limit,
        }
    }
}

impl<Act, S, I> bevy::prelude::Command for InputSequenceBuilder<Act, S, I>
where
    Act: Send + Sync + 'static,
    S: System<In = I, Out = ()> + Send + Sync + 'static,
    I: SystemInput + Send + Sync + 'static,
{
    fn apply(self, world: &mut World) {
        let act = self.build(world);
        let system_entity = act.system_id.entity();
        let id = world.spawn(act).id();
        world.entity_mut(system_entity).insert(ChildOf(id));
    }
}

impl<Act, S, I> bevy::ecs::system::EntityCommand for InputSequenceBuilder<Act, S, I>
where
    Act: Send + Sync + 'static,
    S: System<In = I, Out = ()> + Send + Sync + 'static,
    I: SystemInput + Send + Sync + 'static,
{
    fn apply(self, mut entity_world: EntityWorldMut) {
        let id = entity_world.id();
        entity_world.world_scope(move |world: &mut World| {
            let act = self.build(world);
            let system_entity = act.system_id.entity();
            let mut entity = world.get_entity_mut(id).unwrap();
            entity.insert(act);
            world.entity_mut(system_entity).insert(ChildOf(id));
        });
    }
}

impl<Act, In: SystemInput + Send + Sync + 'static> InputSequence<Act, In>
where
    In: 'static,
{
    /// Create new input sequence. Not operant until added to an entity.
    #[inline(always)]
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T, C, M>(
        system: C,
        acts: impl IntoIterator<Item = T>,
    ) -> InputSequenceBuilder<Act, C::System, In>
    where
        C: IntoCondSystem<In, (), M> + 'static,
        Act: From<T>,
    {
        let mut builder = InputSequenceBuilder::new(system);
        builder.acts = Vec::from_iter(acts.into_iter().map(Act::from));
        builder
    }
}

/// Represents a key sequence
pub type KeySequence = InputSequence<KeyChord, ()>;
/// Represents a key sequence builder
pub type KeySequenceBuilder = InputSequenceBuilder<KeyChord, (), ()>;

/// Represents a gamepad button sequence
pub type ButtonSequence = InputSequence<GamepadButton, In<Entity>>;
