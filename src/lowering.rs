use crate::{facts::{DefValidSourceProgram, ValidSourceProgram}, framework::Process};

pub struct InputAST;

impl DefValidSourceProgram for InputAST {}

impl Process for InputAST {
    type Requires = ();
    type Provides = ValidSourceProgram;

    fn run(self, _input: ()) -> ValidSourceProgram {
        ValidSourceProgram::new::<Self>(&self)
    }
}

