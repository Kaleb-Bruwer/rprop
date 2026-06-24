use crate::framework::propose;

propose!(
    /// The given program is well formed in its language
    ValidSourceProgram,
);

propose!(
    /// If the language has a notion of field order, the order is provided
    FieldOrder,
);

propose!(
    /// At every point in the program, we know which locations the source names are bound to
    LocNameBindings,
);

propose!(
    /// Every return only has a single exit point, no diverging execution paths
    SingleExit,
);

propose!(
    /// All functions are pure; no side effects or external reads
    PureSignatures = InternalPureSignatures && ExternStateInSignatures,
);

propose!(
    /// All functions are pure, not accounting for ExternState
    InternalPureSignatures,
);

propose!(
    /// ExternState is in function signatures
    ExternStateInSignatures,
);

propose!(
    /// There is no dead code; every statement is executed
    NoDeadCode,
);

propose!(
    /// Call substitutions have been resolved
    ResolvedSubstitutions,
);

propose!(
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
