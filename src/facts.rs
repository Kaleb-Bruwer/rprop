use crate::define_fact;

/// The given program is well formed in its language
define_fact!(ValidSourceProgram, DefValidSourceProgram);

/// At every point in the program, we know which locations the source names are bound to
pub struct LocNameBindings {
    _private: (),
}

/// Every return only has a single exit point, no diverging execution paths
pub struct SingleExit {
    _private: (),
}

/// All functions are pure; no side effects or external reads
pub struct PureSignatures{
    _private: (),
}

pub struct NoDeadCode {
    _private: (),
}