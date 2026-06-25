use rprop::{claim, propose, prove};

use crate::{
    components::{Boiler, Condenser, Engine},
    consumables::{Fuel, Seawater},
    physics::{Energy, Water},
    stages::{boiler_stage, engine_stage},
};

pub mod components;
pub mod consumables;
pub mod physics;
pub mod stages;

fn main() {}

propose!(Steamer = Boiler && Condenser && Engine);
propose!(PrimedSteamer = Steamer && Water && Seawater);

claim!(SteamPower = PrimedSteamer && Fuel -> Energy);

#[prove(SteamPower)]
fn steam_power(primed_steamer: PrimedSteamer, fuel: Fuel) -> Energy {
    let PrimedSteamer { steamer: Steamer { boiler, engine, .. }, water, .. } = primed_steamer;

    let pressurized_steam = boiler_stage(boiler, fuel, water);
    let (_, energy) = engine_stage(engine, pressurized_steam);

    energy
}
