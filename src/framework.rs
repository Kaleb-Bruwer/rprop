pub trait Prop {}

pub trait AtomicProp: Prop {}
pub trait Conjunction: Prop {}
pub trait Disjunction: Prop {}

/// Introduce a proposition without proof, recommended for atomic propositions only
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

pub use crate::define_atomic_prop;
pub use crate::take;
#[allow(unused_imports)]
pub use rose_architecture_macros::{define_conjunction, define_disjunction, propose};

#[macro_export]
macro_rules! define_atomic_prop {
    (
        $(#[$doc:meta])*
        $prop:ident $(,)?
    ) => {
        $(#[$doc])*
        #[derive(Clone, Copy)]
        pub struct $prop {
            _private: (),
        }

        impl $crate::framework::Prop for $prop {}

        impl $prop {
            pub(crate) fn new<P: $crate::framework::ProvideProp<Self>>(_provider: &P) -> Self {
                Self { _private: () }
            }
        }
    };
}

#[macro_export]
macro_rules! take {
    ($set:expr, $prop:ident) => {
        $crate::framework::HasProp::<$prop>::prop(&$set)
    };
}
