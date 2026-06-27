use rprop::{claim, propose, prove, Nat};

propose!(At<N>);
propose!(AtThree = At<3>);

claim!(Id<N> = At<N> -> At<N>);
claim!(AtThreeRefl = At<3> -> At<3>);

fn main() {}

#[prove(Id)]
fn id<const N: Nat>(at: At<N>) -> At<N> {
    at
}

#[prove(AtThreeRefl)]
fn at_three_refl(at: At<3>) -> At<3> {
    at
}
