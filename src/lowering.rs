use crate::{
    facts::{FieldOrder, PureSignatures, SingleExit, ValidSourceProgram},
    framework::{Process, ProveFact, define_fact_set, take},
};

pub struct GetAST;

define_fact_set!(SourceAST, [ValidSourceProgram, FieldOrder]);

impl ProveFact<ValidSourceProgram> for GetAST {}
impl ProveFact<FieldOrder> for GetAST {}

impl Process for GetAST {
    type Requires = ();
    type Provides = SourceAST;

    fn run(self, _input: ()) -> SourceAST {
        SourceAST::new(
            ValidSourceProgram::new::<Self>(&self),
            FieldOrder::new::<Self>(&self),
        )
    }
}

pub struct KirBuilder;

define_fact_set!(Kir1, [SingleExit, PureSignatures, FieldOrder]);

impl ProveFact<SingleExit> for KirBuilder {}
impl ProveFact<PureSignatures> for KirBuilder {}

impl Process for KirBuilder {
    type Requires = SourceAST;
    type Provides = Kir1;

    fn run(self, input: SourceAST) -> Kir1 {
        Kir1::new(
            SingleExit::new::<Self>(&self),
            PureSignatures::new::<Self>(&self),
            take!(input, FieldOrder),
        )
    }
}
