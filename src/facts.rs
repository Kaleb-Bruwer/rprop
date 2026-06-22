use crate::framework::{define_atomic_fact, propose};

define_atomic_fact!(
    /// The given program is well formed in its language
    ValidSourceProgram,
);

define_atomic_fact!(
    /// If the language has a notion of field order, the order is provided
    FieldOrder,
);

define_atomic_fact!(
    /// At every point in the program, we know which locations the source names are bound to
    LocNameBindings,
);

define_atomic_fact!(
    /// Every return only has a single exit point, no diverging execution paths
    SingleExit,
);

propose!(
    /// All functions are pure; no side effects or external reads
    PureSignatures = InternalPureSignatures && ExternStateInSignatures,
);

define_atomic_fact!(
    /// All functions are pure, not accounting for ExternState
    InternalPureSignatures,
);

define_atomic_fact!(
    /// ExternState is in function signatures
    ExternStateInSignatures,
);

define_atomic_fact!(
    /// There is no dead code; every statement is executed
    NoDeadCode,
);

define_atomic_fact!(
    /// Call substitutions have been resolved
    ResolvedSubstitutions,
);

define_atomic_fact!(
    /// Struct fields are only identified by their names, never positions
    NoNumberedFields,
);

propose!(
    /// Numbered fields have been switched to named fields
    NumberedFieldsRenamed = FieldOrder,
);

propose!(
    /// Struct fields are in canonized form
    StructCanon = NoNumberedFields || NumberedFieldsRenamed,
);
