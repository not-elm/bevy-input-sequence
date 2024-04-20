use crate::{
    time_limit::TimeLimit,
    KeyChord,
};
use bevy::{
    ecs::{
        entity::Entity,
        system::{BoxedSystem, Commands, IntoSystem, SystemId, ReadOnlySystem, In, System},
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

pub struct InputSequenceBuilder<Act, In> {
    pub system: CondSystem<In, ()>,
    /// Sequence of acts that trigger input sequence
    pub acts: Vec<Act>,
    /// Optional time limit after first match
    pub time_limit: Option<TimeLimit>,
}

pub struct CondSystem<I, O> {
    system: BoxedSystem<I, O>,
    condition: Option<BoxedCondition>,
}

// struct CondMarker;

pub trait IntoCondSystem<I, O, M1, M2> : IntoSystem<I, O, M1> {
    // type System: System<In = I, Out = O>;
    fn into_cond_system(this: Self) -> CondSystem<I, O>;
    // {
    //     CondSystem {
    //         system: IntoSystem::into_system(this),
    //         condition: None,
    //     }
    // }

    fn only_if<P>(self, condition: impl Condition<P> + ReadOnlySystem)
                 -> CondSystem<I, O> where Self: Sized {
        let x = IntoCondSystem::into_cond_system(self);
        if x.condition.is_none() {
            panic!("Cannot chain run_if conditions.");
        }
        CondSystem {
            system: x.system,//IntoSystem::into_system(self),
            condition: Some(Box::new(IntoSystem::into_system(condition))),
        }
    }
}

pub struct Blanket;

impl<I, O, M, T> IntoCondSystem<I, O, M, Blanket> for T where T: IntoSystem<I, O, M> + ?Sized {
    fn into_cond_system(this: Self) -> CondSystem<I, O> {
        CondSystem {
            system: Box::new(IntoSystem::into_system(this)),
            condition: None,
        }
    }
}

pub struct CondSys;
impl<I, O> IntoSystem<I, O, CondSys> for CondSystem<I, O> {
    type System = Box<dyn System<In = I, Out = O>>;//BoxedSystem<I, O>;
    fn into_system(this: Self) -> Self::System {
        this.system
    }
}

impl<Act, In> InputSequenceBuilder<Act, In>
where
    In: 'static,
{
    /// Create new input sequence. Not operant until added to an entity.
    pub fn new<S, P, M>(system: S) -> Self
    where
        S: IntoCondSystem<In, (), P, M> + 'static,
    {
        InputSequenceBuilder {
            acts: Vec::new(),
            system: IntoCondSystem::into_cond_system(system),
            time_limit: None,
        }
    }

    /// Specify a time limit from the start of the first matching input.
    pub fn time_limit(mut self, time_limit: impl Into<TimeLimit>) -> Self {
        self.time_limit = Some(time_limit.into());
        self
    }

    pub fn build(self, world: &mut World) -> InputSequence<Act, In>
    where In: 'static + Sync + Send {
        let consequent = world.register_boxed_system::<In, ()>(self.system.system);
        InputSequence {
            system_id: world.register_system(run_if_impl(self.system.condition, consequent)),
            acts: self.acts,
            time_limit: self.time_limit,
        }
    }
}

fn run_if_impl<I>(
    mut condition: Option<BoxedCondition>,
    consequent: SystemId<I>,
) -> impl FnMut(In<I>, Commands, &World) where I: Send + Sync + 'static {
    move |In(input): In<I>, mut commands: Commands, world: &World| {
        match condition {
            Some(ref mut condition) => {
                if condition.run_readonly((), world) {
                    commands.run_system_with_input(consequent, input);
                }
            }
            None => {
                commands.run_system_with_input(consequent, input);
            }
        }
    }
}

impl<Act, In> bevy::ecs::system::Command for InputSequenceBuilder<Act, In>
where
    Act: Send + Sync + 'static,
    In: Send + Sync + 'static,
{
    fn apply(self, world: &mut World) {
        let act = self.build(world);
        world.spawn(act);
    }
}

impl<Act, In> bevy::ecs::system::EntityCommand for InputSequenceBuilder<Act, In>
where
    Act: Send + Sync + 'static,
    In: Send + Sync + 'static,
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
    pub fn new<T, S, P, M>(system: S, acts: impl IntoIterator<Item = T>) -> InputSequenceBuilder<Act, In>
    where
        Act: From<T>,

        S: IntoCondSystem<In, (), P, M> + 'static,
        // S: IntoSystem<In, (), P> + 'static,
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
