pub trait Prop {}

pub trait AtomicProp: Prop {}
pub trait Conjunction: Prop {}
pub trait Disjunction: Prop {}

/// Bypass any proof requirements and provide a proposition directly
pub trait Sorry {
    fn sorry() -> Self;
}

impl<A, B> Sorry for fn(A) -> B
where
    B: Sorry,
{
    fn sorry() -> Self {
        |_a: A| B::sorry()
    }
}

/// Introduce a proposition without proof, only allowed for atomic propositions
/// If you need this for a non-atomic, introduce an intermediate atomic which implies the proposition
/// i.e. `A && B` can be provided by ProvideProp<C> with `C -> A && B`
pub trait ProvideProp<P: Prop> {}

/// Allows an artifact to contain a proposition
pub trait HasProp<F: Prop>: Conjunction {
    fn prop(&self) -> F;
}

pub trait Process {
    type Requires;
    type Provides;

    fn run(self, input: Self::Requires) -> Self::Provides;
}

// pub use crate::define_atomic_prop;
pub use crate::take;
#[allow(unused_imports)]
pub use rose_architecture_macros::{claim, define_conjunction, define_disjunction, propose};

#[macro_export]
macro_rules! take {
    ($set:expr, $prop:ident) => {
        $crate::framework::HasProp::<$prop>::prop(&$set)
    };
}
