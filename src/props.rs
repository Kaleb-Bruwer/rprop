use crate::framework::{define_atomic_prop, propose};

define_atomic_prop!(
    /// The given program is well formed in its language
    ValidSourceProgram,
);

define_atomic_prop!(
    /// If the language has a notion of field order, the order is provided
    FieldOrder,
);

define_atomic_prop!(
    /// At every point in the program, we know which locations the source names are bound to
    LocNameBindings,
);

define_atomic_prop!(
    /// Every return only has a single exit point, no diverging execution paths
    SingleExit,
);

propose!(
    /// All functions are pure; no side effects or external reads
    PureSignatures = InternalPureSignatures && ExternStateInSignatures,
);

define_atomic_prop!(
    /// All functions are pure, not accounting for ExternState
    InternalPureSignatures,
);

define_atomic_prop!(
    /// ExternState is in function signatures
    ExternStateInSignatures,
);

define_atomic_prop!(
    /// There is no dead code; every statement is executed
    NoDeadCode,
);

define_atomic_prop!(
    /// Call substitutions have been resolved
    ResolvedSubstitutions,
);

define_atomic_prop!(
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
