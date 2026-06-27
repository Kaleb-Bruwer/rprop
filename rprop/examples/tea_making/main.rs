use rprop::{claim, propose, prove, take};

propose!(TapWater);
propose!(BottledWater);

propose!(HasWater = TapWater || BottledWater);
propose!(Kettle);
propose!(BoiledWater = Kettle && HasWater);

propose!(Teabag<N>);
propose!(Cup);

propose!(Sugar);
propose!(Milk);

propose!(
    ///Represet the combinations of consumables that can be used to make tea
    Consumables<N> = Teabag<N> || Teabag<N> && Sugar || Teabag<N> && Milk || Teabag<N> && Sugar && Milk
);

propose!(Tea = Consumables<1> && Cup && BoiledWater);

claim!(TeaFromTap = Teabag<1> && Cup && TapWater && Kettle -> Tea);
claim!(AlwaysNeedTeabag = Tea -> Teabag<1>);

fn main() {}

/// Proof that tea can be made with a teabag, cup, tap water and kettle
#[prove(TeaFromTap)]
fn tea_from_tap(teabag: Teabag<1>, cup: Cup, tap_water: TapWater, kettle: Kettle) -> Tea {
    let consumables_1: Consumables<1> = teabag.into();
    let boiled_water = BoiledWater { kettle, has_water: tap_water.into() };

    Tea { consumables_1, cup, boiled_water }
}

/// Prove by case analysis that each combination of consumables requires a teabag
#[prove(AlwaysNeedTeabag)]
fn always_need_teabag(tea: Tea) -> Teabag<1> {
    let consumables_1 = tea.consumables_1;

    match consumables_1 {
        Consumables::Teabag(teabag) => teabag,
        // Syntax alternative for conjunction members
        Consumables::Consumables0(consumables_0) => consumables_0.teabag_n,
        Consumables::Consumables1(Consumables1 { teabag_n, .. }) => teabag_n,
        Consumables::Consumables2(Consumables2 { teabag_n, .. }) => teabag_n,
    }
}
