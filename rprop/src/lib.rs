pub trait Prop {}

pub trait AtomicProp: Prop {}
pub trait Conjunction: Prop {}
pub trait Disjunction: Prop {}

/// Alternate syntax for accesing the components of a conjunction
pub trait HasProp<F>: Conjunction {
    fn prop(&self) -> F;
}

#[allow(unused_imports)]
pub use rprop_derive::{claim, define_conjunction, define_disjunction, propose, prove};

#[macro_export]
macro_rules! take {
    ($set:expr, $prop:ident) => {
        $crate::HasProp::<$prop>::prop(&$set)
    };
}

pub enum Absurd {}

pub fn ex_falso<P: Prop>(absurd: Absurd) -> P {
    match absurd {}
}

pub type Nat = u32; // Keep in sync with rprop_derive/src/nat.rs
