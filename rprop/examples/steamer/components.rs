use rprop::propose;

use crate::consumables::{Fuel, Seawater};
use crate::physics::{BoilWaterEnclosed, CondenseSteam, Coolant, ExpandSteam, Heat};

propose!(BurnFuel = Fuel -> Heat);
propose!(
    /// The boiler allows us to burn fuel and boil water, producing pressurized steam
    Boiler = BurnFuel && BoilWaterEnclosed
);

propose!(
    /// The engine harnesses energy from the expansion of presurized steam
    Engine = ExpandSteam
);

propose!(
    /// The condenser allows us to use seawater as a coolant
    CoolWithSeawater = Seawater -> Coolant
);

propose!(
    /// The condenser turns steam back into water using seawater as a coolant
    Condenser = CondenseSteam && CoolWithSeawater
);
