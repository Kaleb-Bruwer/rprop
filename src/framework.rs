pub trait Fact {}

pub trait HasFact<F: Fact> {
    fn fact(&self) -> F;
}

pub trait Process {
    type Requires;
    type Provides;

    fn run(self, input: Self::Requires) -> Self::Provides;
}

pub use crate::define_fact;
pub use crate::take;
pub use rose_architecture_macros::define_fact_set;

#[macro_export]
macro_rules! define_fact {
    (
        $(#[$doc:meta])*
        $fact:ident,
        $provider_trait:ident $(,)?
    ) => {
        $(#[$doc])*
        #[derive(Clone, Copy)]
        pub struct $fact {
            _private: (),
        }

        impl $crate::framework::Fact for $fact {}

        pub(crate) trait $provider_trait {}

        impl $fact {
            pub(crate) fn new<P: $provider_trait>(_provider: &P) -> Self {
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
