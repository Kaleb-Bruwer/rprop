pub trait Fact {}

pub trait AtomicFact: Fact {}

pub trait CompositeFact: Fact {}

/// Allows a process to introduce a fact
pub trait ProveFact<F: Fact> : Process {}

/// Allows an artifact to contain a fact
pub trait HasFact<F: Fact> : CompositeFact{
    fn fact(&self) -> F;
}

pub trait Process {
    type Requires;
    type Provides;

    fn run(self, input: Self::Requires) -> Self::Provides;
}

pub use crate::define_atomic_fact;
pub use crate::take;
pub use crate::define_fact_set;
pub use rose_architecture_macros::define_fact_set as define_fact_set_inner;


#[macro_export]
macro_rules! define_fact_set {
    (
        $(#[$doc:meta])*
        $name:ident,
        [$($fact:ident),* $(,)?] $(,)?
    ) => {
        $crate::framework::define_fact_set_inner!(
            $(#[$doc])*
            $name,
            [$($fact),*]
        );

        impl $crate::framework::Fact for $name {}
        impl $crate::framework::CompositeFact for $name {}
    };
}

#[macro_export]
macro_rules! define_atomic_fact {
    (
        $(#[$doc:meta])*
        $fact:ident $(,)?
    ) => {
        $(#[$doc])*
        #[derive(Clone, Copy)]
        pub struct $fact {
            _private: (),
        }

        impl $crate::framework::Fact for $fact {}

        impl $fact {
            pub(crate) fn new<P: $crate::framework::ProveFact<Self>>(_provider: &P) -> Self {
                Self { _private: () }
            }
        }
    };
}

#[macro_export]
macro_rules! take {
    ($set:expr, $fact:ident) => {
        $crate::framework::HasFact::<$fact>::fact(&$set)
    };
}
