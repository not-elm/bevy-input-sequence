use bevy::ecs::system::{Combine, CombinatorSystem, System};

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
