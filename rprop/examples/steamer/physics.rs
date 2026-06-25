use rprop::propose;

propose!(Water);
propose!(PressurizedSteam);
propose!(Steam);

propose!(Energy);

//Intellectually lazy way to avoid modelling temperature
propose!(Heat);
propose!(Coolant);

propose!(
    /// Apply heat to boil water
    BoilWater = Water && Heat -> Steam
);

propose!(
    /// Apply heat in a pressure vessel to produce pressurized steam
    BoilWaterEnclosed = Water && Heat -> PressurizedSteam
);

propose!(
    /// Steam gives up energy as it expands
    ExpandSteam = PressurizedSteam -> Steam && Energy
);

propose!(
    /// Draw heat out of the steam to condense it back into water
    CondenseSteam = Steam && Coolant -> Water
);
