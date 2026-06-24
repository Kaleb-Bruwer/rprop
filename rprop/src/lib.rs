pub trait Prop {}

pub trait AtomicProp: Prop {}
pub trait Conjunction: Prop {}
pub trait Disjunction: Prop {}

/// Alternate syntax for accesing the components of a conjunction
pub trait HasProp<F: Prop>: Conjunction {
    fn prop(&self) -> F;
}

#[allow(unused_imports)]
pub use rprop_macros::{claim, define_conjunction, define_disjunction, propose};

#[macro_export]
macro_rules! take {
    ($set:expr, $prop:ident) => {
        $crate::HasProp::<$prop>::prop(&$set)
    };
}
