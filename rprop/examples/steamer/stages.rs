use rprop::{claim, prove, take};

use crate::components::{Boiler, BurnFuel, Condenser, Engine};
use crate::consumables::{Fuel, Seawater};
use crate::physics::{
    BoilWaterEnclosed, BoilWaterEnclosed0, CondenseSteam0, Energy, ExpandSteam0, PressurizedSteam, Steam, Water,
};

claim!(BoilerStage = Boiler && Fuel && Water -> PressurizedSteam);
claim!(CondenserStage = Condenser && Steam && Seawater -> Water);
claim!(EngineStage = Engine && PressurizedSteam -> Steam && Energy);

#[prove(BoilerStage)]
pub fn boiler_stage(inputs: BoilerStage0) -> PressurizedSteam {
    let BoilerStage0 { boiler, fuel, water } = inputs;

    let burn = take!(boiler, BurnFuel);
    let boil = take!(boiler, BoilWaterEnclosed);

    let heat = burn(fuel);
    let steam = boil(BoilWaterEnclosed0 { water, heat });
    steam
}

#[prove(CondenserStage)]
pub fn condenser_stage(inputs: CondenserStage0) -> Water {
    let CondenserStage0 { condenser, steam, seawater } = inputs;

    //Condenser allows seawater as a coolant
    let coolant = (condenser.cool_with_seawater)(seawater);
    // We can condense steam with any coolant
    let water = (condenser.condense_steam)(CondenseSteam0 { steam, coolant });

    water
}

#[prove(EngineStage)]
pub fn engine_stage(inputs: EngineStage0) -> EngineStage1 {
    let EngineStage0 { engine, pressurized_steam } = inputs;

    let ExpandSteam0 { steam, energy } = (engine.expand_steam)(pressurized_steam);
    EngineStage1 { steam, energy }
}
