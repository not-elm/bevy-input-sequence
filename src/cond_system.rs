use bevy::ecs::system::{CombinatorSystem, Combine, IntoSystem, System};
use std::borrow::Cow;

pub trait IntoCondSystem<I, O, M>: IntoSystem<I, O, M> {
    fn only_if<B, MarkerB>(self, system: B) -> SilentCondSystem<Self::System, B::System>
    where
        B: IntoSystem<(), bool, MarkerB>,
    {
        let system_a = IntoSystem::into_system(self);
        let system_b = IntoSystem::into_system(system);
        let name = format!("Cond({}, {})", system_a.name(), system_b.name());
        SilentCondSystem::new(system_a, system_b, Cow::Owned(name))
    }
}

impl<I, O, M, T> IntoCondSystem<I, O, M> for T where T: IntoSystem<I, O, M> + ?Sized {}

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

    fn combine(
        input: Self::In,
        a: impl FnOnce(A::In) -> A::Out,
        b: impl FnOnce(B::In) -> B::Out,
    ) -> Self::Out {
        b(()).then(|| a(input))
    }
}

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

    fn combine(
        input: Self::In,
        a: impl FnOnce(A::In) -> A::Out,
        b: impl FnOnce(B::In) -> B::Out,
    ) -> Self::Out {
        if b(()) {
            a(input);
        }
    }
}
