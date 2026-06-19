use crate::framework::define_fact;

define_fact!(
    /// The given program is well formed in its language
    ValidSourceProgram,
);

define_fact!(
    /// If the language has a notion of field order, the order is provided
    FieldOrder,
);

define_fact!(
    /// At every point in the program, we know which locations the source names are bound to
    LocNameBindings,
);

define_fact!(
    /// Every return only has a single exit point, no diverging execution paths
    SingleExit,
);

define_fact!(
    /// All functions are pure; no side effects or external reads
    PureSignatures,
);

define_fact!(
    /// There is no dead code; every statement is executed
    NoDeadCode,
);
