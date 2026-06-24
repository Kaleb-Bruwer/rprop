use rprop::{claim, propose};

propose!(TapWater);
propose!(BottledWater);

propose!(HasWater = TapWater || BottledWater);
propose!(Kettle);
propose!(BoiledWater = Kettle && HasWater);

propose!(Teabag);
propose!(Cup);
propose!(Tea);

propose!(Sugar);
propose!(Milk);

propose!(
    ///Represet the combinations of consumables that can be used to make tea
    Consumables = Teabag || Teabag && Sugar || Teabag && Milk || Teabag && Sugar && Milk);
propose!(MakeTea = Consumables && Cup && BoiledWater -> Tea);

claim!(TeaFromTap = Teabag && Cup && TapWater && Kettle -> Tea);
claim!(AlwaysNeedTeabag = Consumables -> Teabag);

fn main() {}

// Proof that tea can be made with a teabag, cup, tap water and kettle
fn tea_from_tap(components: TeaFromTap_0) -> Tea {
    let TeaFromTap_0 { teabag, cup, tap_water, kettle } = components;

    let consumables: Consumables = teabag.into();
    let boiled_water = BoiledWater { kettle, has_water: tap_water.into() };

    let make_tea_args = MakeTea_0 { consumables, cup, boiled_water };
    todo!("Invoking a proposed implication still needs to be implemented")
}

/// Prove by case analysis that each combination of consumables requires a teabag
fn always_need_teabag(consumables: Consumables) -> Teabag {
    match consumables {
        Consumables::Teabag(teabag) => teabag,
        Consumables::Consumables_0(Consumables_0 { teabag, .. }) => teabag,
        Consumables::Consumables_1(Consumables_1 { teabag, .. }) => teabag,
        Consumables::Consumables_2(Consumables_2 { teabag, .. }) => teabag,
    }
}
