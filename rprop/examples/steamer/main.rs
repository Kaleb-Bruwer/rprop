use rprop::{claim, propose, prove};

use crate::{
    components::{Boiler, Condenser, Engine},
    consumables::{Fuel, Seawater},
    physics::{Energy, Water},
    stages::{BoilerStage0, EngineStage0, EngineStage1, boiler_stage, engine_stage},
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
fn steam_power(inputs: SteamPower0) -> Energy {
    let SteamPower0 { primed_steamer, fuel } = inputs;

    let PrimedSteamer { steamer: Steamer { boiler, engine, .. }, water, .. } = primed_steamer;

    let boiler_input = BoilerStage0 { boiler, fuel, water };
    let pressurized_steam = boiler_stage(boiler_input);

    let engine_input = EngineStage0 { engine, pressurized_steam };
    let EngineStage1 { energy, .. } = engine_stage(engine_input);

    energy
}
