use rprop::{claim, propose, prove, Absurd};

fn main() {}

propose!(Hot);
propose!(Cold = !Hot); //Cold = Hot -> Absurd

claim!(OnlyHotOrCold = Hot && Cold -> Absurd);

#[prove(OnlyHotOrCold)]
fn only_hot_or_cold(hot: Hot, cold: Cold) -> Absurd {
    cold(hot)
}