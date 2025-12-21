//! Extend [IntoSystem] for conditional execution
use bevy::ecs::system::{CombinatorSystem, Combine, IntoSystem, System, SystemIn, SystemInput};
use bevy::prelude::DebugName;

/// Extend [IntoSystem] to allow for some conditional execution. Probably only
/// appropriate for one-shot systems. Prefer
/// [`run_if()`](bevy::ecs::schedule::IntoScheduleConfigs::run_if()) when directly
/// adding to the scheduler.
pub trait IntoCondSystem<I, O, M>: IntoSystem<I, O, M>
where
    I: SystemInput,
{
    /// Only run self's system if the given `system` parameter returns true. No
    /// output is provided. (This is convenient for running systems with
    /// [bevy::prelude::Commands::run_system]).
    fn only_if<B, MarkerB>(self, system: B) -> SilentCondSystem<Self::System, B::System>
    where
        B: IntoSystem<(), bool, MarkerB>,
    {
        let system_a = IntoSystem::into_system(self);
        let system_b = IntoSystem::into_system(system);
        let name = format!("SilentCond({}, {})", system_a.name(), system_b.name());
        SilentCondSystem::new(system_a, system_b, DebugName::owned(name))
    }

    /// Only run self's system if the given `system` parameter returns true. The
    /// output is an `Option<Self::Out>`. `None` is returned when the condition
    /// returns false.
    fn only_if_with_output<B, MarkerB>(self, system: B) -> CondSystem<Self::System, B::System>
    where
        B: IntoSystem<(), bool, MarkerB>,
    {
        let system_a = IntoSystem::into_system(self);
        let system_b = IntoSystem::into_system(system);
        let name = format!("Cond({}, {})", system_a.name(), system_b.name());
        CondSystem::new(system_a, system_b, DebugName::owned(name))
    }
}

impl<I, O, M, T> IntoCondSystem<I, O, M> for T
where
    T: IntoSystem<I, O, M>,
    I: SystemInput,
{
}

/// A one-shot conditional system comprised of consequent `SystemA` and
/// conditional `SystemB`.
pub type CondSystem<SystemA, SystemB> = CombinatorSystem<Cond, SystemA, SystemB>;

#[doc(hidden)]
pub struct Cond;

impl<A, B> Combine<A, B> for Cond
where
    B: System<In = (), Out = bool>,
    A: System,
{
    type In = A::In;
    type Out = Option<A::Out>;

    fn combine<T>(
        input: <Self::In as SystemInput>::Inner<'_>,
        data: &mut T,
        a: impl FnOnce(SystemIn<'_, A>, &mut T) -> Result<A::Out, bevy::ecs::system::RunSystemError>,
        b: impl FnOnce(SystemIn<'_, B>, &mut T) -> Result<B::Out, bevy::ecs::system::RunSystemError>,
    ) -> Result<Self::Out, bevy::ecs::system::RunSystemError> {
        let condition = b((), data)?;
        if condition {
            Ok(Some(a(input, data)?))
        } else {
            Ok(None)
        }
    }
}

/// A one-shot conditional system comprised of consequent `SystemA` and
/// conditional `SystemB` with no output.
pub type SilentCondSystem<SystemA, SystemB> = CombinatorSystem<SilentCond, SystemA, SystemB>;

#[doc(hidden)]
pub struct SilentCond;

impl<A, B> Combine<A, B> for SilentCond
where
    B: System<In = (), Out = bool>,
    A: System,
{
    type In = A::In;
    type Out = ();

    fn combine<T>(
        input: <Self::In as SystemInput>::Inner<'_>,
        data: &mut T,
        a: impl FnOnce(SystemIn<'_, A>, &mut T) -> Result<A::Out, bevy::ecs::system::RunSystemError>,
        b: impl FnOnce(SystemIn<'_, B>, &mut T) -> Result<B::Out, bevy::ecs::system::RunSystemError>,
    ) -> Result<Self::Out, bevy::ecs::system::RunSystemError> {
        let condition = b((), data)?;
        if condition {
            a(input, data)?;
        }
        Ok(())
    }
}
