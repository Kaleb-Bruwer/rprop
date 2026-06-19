pub trait Fact {}

pub trait HasFact<F: Fact> {
    fn fact(&self) -> F;
}

pub trait Process {
    type Requires;
    type Provides;

    fn run(self, input: Self::Requires) -> Self::Provides;
}

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

// We allow non_snake_case here to match the fact names
#[macro_export]
macro_rules! define_fact_set {
    (
        $(#[$doc:meta])+
        $name:ident,
        [$($fact:ident),* $(,)?]
    ) => {
        $(#[$doc])*
        $crate::define_fact_set!(@body $name, $($fact),*);
    };

    (
        $name:ident,
        [$($fact:ident),* $(,)?]
    ) => {
        $crate::define_fact_set!(@body $name, $($fact),*);
    };

    (@body $name:ident, $($fact:ident),* $(,)?) => {
        #[allow(non_snake_case)]
        pub struct $name {
            $( pub $fact: $fact, )*
        }

        impl $name {
            pub fn new($($fact: $fact),*) -> Self {
                Self {
                    $($fact),*
                }
            }
        }

        $(
            impl $crate::framework::HasFact<$fact> for $name {
                fn fact(&self) -> $fact {
                    self.$fact
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! take {
    ($set:expr, $fact:ident) => {
        $crate::framework::HasFact::<$fact>::fact(&$set)
    };
}
