pub trait Fact {}

pub trait Process {
    type Requires;
    type Provides;

    fn run(self, input: Self::Requires) -> Self::Provides;
}

#[macro_export]
macro_rules! define_fact {
    ($fact:ident, $provider_trait:ident) => {
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