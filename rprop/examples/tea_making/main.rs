use rprop::{claim, propose, take};

propose!(TapWater);
propose!(BottledWater);

propose!(HasWater = TapWater || BottledWater);
propose!(Kettle);
propose!(BoiledWater = Kettle && HasWater);

propose!(Teabag);
propose!(Cup);

propose!(Sugar);
propose!(Milk);

propose!(
    ///Represet the combinations of consumables that can be used to make tea
    Consumables = Teabag || Teabag && Sugar || Teabag && Milk || Teabag && Sugar && Milk
);

propose!(Tea = Consumables && Cup && BoiledWater);

claim!(TeaFromTap = Teabag && Cup && TapWater && Kettle -> Tea);
claim!(AlwaysNeedTeabag = Consumables -> Teabag);

fn main() {}

// Proof that tea can be made with a teabag, cup, tap water and kettle
fn tea_from_tap(components: TeaFromTap0) -> Tea {
    let TeaFromTap0 { teabag, cup, tap_water, kettle } = components;

    let consumables: Consumables = teabag.into();
    let boiled_water = BoiledWater { kettle, has_water: tap_water.into() };

    Tea { consumables, cup, boiled_water }
}

/// Prove by case analysis that each combination of consumables requires a teabag
fn always_need_teabag(consumables: Consumables) -> Teabag {
    match consumables {
        Consumables::Teabag(teabag) => teabag,
        // Syntax alternative for conjunction members
        Consumables::Consumables0(consumables_0) => take!(consumables_0, Teabag),
        Consumables::Consumables1(Consumables1 { teabag, .. }) => teabag,
        Consumables::Consumables2(Consumables2 { teabag, .. }) => teabag,
    }
}

impl __rprop_TeaFromTap_proof for TeaFromTap {
    const PROOF: Self = tea_from_tap;
}

impl __rprop_AlwaysNeedTeabag_proof for AlwaysNeedTeabag {
    const PROOF: Self = always_need_teabag;
}
