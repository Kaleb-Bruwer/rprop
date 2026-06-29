# RProp [![Build](https://github.com/Kaleb-Bruwer/rprop/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/Kaleb-Bruwer/rprop/actions/workflows/build.yml) [![Test](https://github.com/Kaleb-Bruwer/rprop/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/Kaleb-Bruwer/rprop/actions/workflows/test.yml)

This library provides propositional logic in Rust, allowing users to use formal reasoning to model systems in a familiar language and environment.

```toml
[dependencies]
rprop = "0.2"
```

## Overview

Logic is built up from atomic propositions, bound together by three logical operations: conjunctions ```&&```, disjunctions ```||``` and implications ```->```, which we model as ```structs```, ```enums``` and ```functions``` respectively. We allow the user to define propositions, claims and proofs. Claims are implications that require proof. Otherwise, a compile-time error will be raised. Proofs are written as functions, using the same signature as the corresponding claim. The proof function must be annotated with ```prove(ClaimName)``` to be recognized.

Proofs are done by construction. All the propositions have private constructors, so they can't be introduced from nothing. For any given proof function, the only assumptions it has are its inputs, and any proven implications (functions) that are within scope. Beyond that, propositions can only be constructed by following their defined logic. A conjunction requires all its members (hence a struct), and a disjunction only one (hence an enum).

## Example

In this example we are declaring propositions to model a kettle that boils water. ```Water``` is defined as being either ```TapWater``` or ```BottledWater```. We then claim that if we can boil ```Water```, we can also boil ```TapWater```, giving us the ```BoilTapWater``` implication. We need to prove our claim, which is why we have the ```boil_tap_water``` function, annotated with ```prove(BoilTapWater)```.


The claim is proved by construction. Note that the proof function must have the same signature as the corresponding claim, which, as an implication, is really a type alias for a function signature. A proof is satisfied if we manage to get the return type correct. Since we cannot construct propositions directly, this forces us to use logic to reason from the inputs.

```rust
use rprop::{claim, propose, prove};

propose!(TapWater); //Atomic proposition
propose!(BottledWater);
propose!(Water = TapWater || BottledWater); //Disjunction
propose!(BoilingWater);

propose!(Kettle);

propose!(BoilWater = Water && Kettle -> BoilingWater); //Conjunction and implication

claim!(BoilTapWater = TapWater && Kettle && BoilWater -> BoilingWater);

#[prove(BoilTapWater)]
pub fn boil_tap_water(tap_water: TapWater, kettle: Kettle, boil: BoilWater) -> BoilingWater {
    let water: Water = tap_water.into(); // Disjunctions have From implementations for each variant
    (boil)(water, kettle) // boil is a function that returns BoilingWater
}

```

More examples can be found [here](rprop/examples/).

## Core concepts

### Propositions

<table>
<tr><th>Proposition</th><th>Purpose</th><th>Example</th></tr>

<tr><td>Atomic</td><td>Simplest piece of information</td><td>propose!(A)</td></tr>
<tr><td>Conjunction</td><td>A conjunction is true if all its 1+ components are true</td><td>propose!(A = B && C && D)</td></tr>
<tr><td>Disjunction</td><td>A disjunction is true if any of its 2+ components are true</td><td>propose!(A = B || C)</td></tr>
<tr><td>Implication</td><td>If we have the lhs, we get the rhs</td><td>propose!(A = B -> C)</td></tr>
</table>

A proposition may use nested logic, in which case intermediate declarations will be made to represent nested terms. This should be done with caution, as you will not control the names given to such nested terms.
```rust
propose!(A = B && (C || D) -> E || F)
```

**Note:** An implication only provides a function signature, not an implementation. If you wish to use an unproven implication in another proof, it has to be one of the inputs (i.e. it has to be an explicit assumption). The ```BoilWater``` implication in the earlier example demonstrates this concept.

### Negation

RProp is based on the [Curry-Howard Correspondence](https://en.wikipedia.org/wiki/Curry%E2%80%93Howard_correspondence), which does not have negation as a primitive construct. Instead, negations are modeled as an implication to ```Absurd```. We do, however, provide ```!``` as syntactic sugar.

```rust
use rprop::{claim, propose, prove, Absurd};

propose!(Hot);
propose!(Cold = !Hot); // Cold = Hot -> Absurd

claim!(OnlyHotOrCold = Hot && Cold -> Absurd);

#[prove(OnlyHotOrCold)]
fn only_hot_or_cold(hot: Hot, cold: Cold) -> Absurd {
    cold(hot) // Negation is a function that produces Absurd if its inverse is provided
}
```

As is demonstrated in the above example, negations become function signatures. If ```Absurd``` can be reached, it proves a contradiction. ```Absurd``` can also be used to prove anything via ```ex_falso```.

### Soundness

Proofs are only meaningful if their inputs are sound. While a best-effort soundness check is planned as a coming feature, ensuring soundness is and will remain the user's responsibility. If contradictory inputs are provided, any conclusion can be reached by using ```ex_falso```.

## Remaining work

This is an early and experimental version of RProp. There are several known opportunities for improvement, which are expanded on in the open issues.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.