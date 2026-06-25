use rprop::{claim, prove, take};

use crate::components::{Boiler, BurnFuel, Condenser, Engine};
use crate::consumables::{Fuel, Seawater};
use crate::physics::{BoilWaterEnclosed, Energy, PressurizedSteam, Steam, Water};

claim!(BoilerStage = Boiler && Fuel && Water -> PressurizedSteam);
claim!(CondenserStage = Condenser && Steam && Seawater -> Water);
claim!(EngineStage = Engine && PressurizedSteam -> Steam && Energy);

#[prove(BoilerStage)]
pub fn boiler_stage(boiler: Boiler, fuel: Fuel, water: Water) -> PressurizedSteam {
    let burn = take!(boiler, BurnFuel);
    let boil = take!(boiler, BoilWaterEnclosed);

    let heat = burn(fuel);
    boil(water, heat)
}

#[prove(CondenserStage)]
pub fn condenser_stage(condenser: Condenser, steam: Steam, seawater: Seawater) -> Water {
    //Condenser allows seawater as a coolant
    let coolant = (condenser.cool_with_seawater)(seawater);
    // We can condense steam with any coolant
    (condenser.condense_steam)(steam, coolant)
}

#[prove(EngineStage)]
pub fn engine_stage(engine: Engine, pressurized_steam: PressurizedSteam) -> (Steam, Energy) {
    (engine.expand_steam)(pressurized_steam)
}
