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
    // type Out = Option<A::Out>;
    type Out = ();

    fn combine(
        input: Self::In,
        a: impl FnOnce(A::In) -> A::Out,
        b: impl FnOnce(B::In) -> B::Out,
    ) -> Self::Out {
        b(()).then(|| a(input));
    }
}
