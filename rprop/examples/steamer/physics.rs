use rprop::propose;

propose!(
    Water; PressurizedSteam; Steam; Energy; Coolant;

    /// Intellectually lazy way to avoid modelling temperature
    Heat;

    /// Apply heat to water to produce steam
    BoilWater = Water && Heat -> Steam;

    /// Apply heat in a pressure vessel to produce pressurized steam
    BoilWaterEnclosed = Water && Heat -> PressurizedSteam;

    /// Steam gives up energy as it expands
    ExpandSteam = PressurizedSteam -> Steam && Energy;

    /// Draw heat out of the steam to condense it back into water
    CondenseSteam = Steam && Coolant -> Water;
);
