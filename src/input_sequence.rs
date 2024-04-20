use crate::{
    time_limit::TimeLimit,
    KeyChord,
    cond_system::CondSystem,
};

use std::borrow::Cow;
use bevy::{
    log::warn,
    ecs::{
        entity::Entity,
        system::{Commands, IntoSystem, SystemId, ReadOnlySystem, In, System},
        schedule::{Condition, BoxedCondition},
        world::World,
    },
    input::gamepad::Gamepad,
    prelude::{Component, GamepadButtonType},
    reflect::Reflect,
};

/// An input sequence is a series of acts that fires an event when matched with
/// inputs within the given time limit.
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

pub struct InputSequenceBuilder<Act, S> {
    pub system: S,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<Act>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
}

// pub struct CondSystem<S> {
//     system: S,
//     condition: Option<BoxedCondition>,
// }

// struct CondMarker;
//

pub trait IntoCondSystem<I, O, M> : IntoSystem<I, O, M> {
    // fn into_cond_system(this: Self) -> CondSystem<S>;
    // {
    //     CondSystem {
    //         system: IntoSystem::into_system(this),
    //         condition: None,
    //     }
    // }


    fn only_if<B, MarkerB>(self, system: B) -> CondSystem<Self::System, B::System>
    where
        B: IntoSystem<(), bool, MarkerB>,
    {
        let system_a = IntoSystem::into_system(self);
        let system_b = IntoSystem::into_system(system);
        let name = format!("Cond({}, {})", system_a.name(), system_b.name());
        CondSystem::new(system_a, system_b, Cow::Owned(name))
    }
    // fn only_if<P>(self, condition: impl Condition<P>)
    //              -> CondSystem<S> where Self: Sized {
    //     let x = IntoCondSystem::into_cond_system(self);
    //     if x.condition.is_some() {
    //         panic!("Cannot chain run_if conditions.");
    //     }
    //     warn!("create cond system");
    //     CondSystem {
    //         system: x.system,
    //         condition: Some(Box::new(IntoSystem::into_system(condition))),
    //     }
    // }
}

pub struct Blanket;

impl<I, O, M, T> IntoCondSystem<I, O, M> for T where T: IntoSystem<I, O, M> + ?Sized {
    // fn into_cond_system(this: Self) -> CondSystem<T::System> {
    //     CondSystem {
    //         system: IntoSystem::into_system(this),
    //         condition: None,
    //     }
    // }
}

// pub struct CondSys;
// impl<S> IntoSystem<S::In, S::Out, CondSys> for CondSystem<S>
// where S: System{
//     type System = S;
//     fn into_system(this: Self) -> Self::System {
//         this.system
//     }
// }

impl<Act> InputSequenceBuilder<Act, ()>
{
    /// Create new input sequence. Not operant until added to an entity.
    pub fn new<C, I, M>(system: C) -> InputSequenceBuilder<Act, C::System>
        where C: IntoCondSystem<I, (), M> + 'static,
              I: Send + Sync + 'static,
    {
        InputSequenceBuilder {
            acts: Vec::new(),
            system: IntoSystem::into_system(system),
            time_limit: None,
        }
    }
}

impl <Act, S> InputSequenceBuilder<Act, S> where S: System<Out = ()>{

    /// Specify a time limit from the start of the first matching input.
    pub fn time_limit(mut self, time_limit: impl Into<TimeLimit>) -> Self {
        self.time_limit = Some(time_limit.into());
        self
    }

    pub fn build(self, world: &mut World) -> InputSequence<Act, S::In> {
        InputSequence {
            system_id: world.register_system(self.system),
            acts: self.acts,
            time_limit: self.time_limit,
        }
    }
}

fn run_if_impl<I>(
    mut condition: BoxedCondition,
    consequent: SystemId<I>,
) -> impl FnMut(In<I>, Commands, &World) where I: Send + Sync + 'static {
    move |In(input): In<I>, mut commands: Commands, world: &World| {
        eprintln!("checking condition");
        if condition.run_readonly((), world) {
        eprintln!("running after condition");
            commands.run_system_with_input(consequent, input);
        }
    }
}

impl<Act, S> bevy::ecs::system::Command for InputSequenceBuilder<Act, S>
where
    Act: Send + Sync + 'static,
    S: System<Out = ()> + Send + Sync + 'static,
    S::In: Send + Sync + 'static,
{
    fn apply(self, world: &mut World) {
        let act = self.build(world);
        world.spawn(act);
    }
}

impl<Act, S> bevy::ecs::system::EntityCommand for InputSequenceBuilder<Act, S>
where
    Act: Send + Sync + 'static,
    S: System<Out = ()> + Send + Sync + 'static,
    S::In: Send + Sync + 'static,
{
    fn apply(self, id: Entity, world: &mut World) {
        let act = self.build(world);
        let mut entity = world.get_entity_mut(id).unwrap();
        entity.insert(act);
    }
}

impl<Act, In> InputSequence<Act, In>
where
    In: 'static,
{
    /// Create new input sequence. Not operant until added to an entity.
    #[inline(always)]
    pub fn new<T, C, I, M>(system: C, acts: impl IntoIterator<Item = T>) -> InputSequenceBuilder<Act, C::System>
    where
        C: IntoCondSystem<I, (), M> + 'static,
        Act: From<T>,
        I: Send + Sync + 'static,
    {
        let mut builder = InputSequenceBuilder::new(system);
        builder.acts = Vec::from_iter(acts.into_iter().map(Act::from));
        builder
    }
}

/// Represents a key sequence.
pub type KeySequence = InputSequence<KeyChord, ()>;

/// Represents a gamepad button sequence.
pub type ButtonSequence = InputSequence<GamepadButtonType, Gamepad>;
