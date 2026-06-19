use crate::{
    facts::{
        ExternStateInSignatures, FieldOrder, InternalPureSignatures, NoNumberedFields, PureSignatures, ResolvedSubstitutions, SingleExit, StructCanon, ValidSourceProgram
    },
    framework::{Process, ProveFact, define_fact_set, take},
};

pub struct GetAST;

impl ProveFact<ValidSourceProgram> for GetAST {}
impl ProveFact<FieldOrder> for GetAST {}

define_fact_set!(SourceAST, [ValidSourceProgram, FieldOrder]);

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

impl ProveFact<SingleExit> for KirBuilder {}
impl ProveFact<InternalPureSignatures> for KirBuilder {}
impl ProveFact<ResolvedSubstitutions> for KirBuilder {}

define_fact_set!(Kir1, [SingleExit, InternalPureSignatures, FieldOrder, ResolvedSubstitutions]);

impl Process for KirBuilder {
    type Requires = SourceAST;
    type Provides = Kir1;

    fn run(self, input: SourceAST) -> Kir1 {
        Kir1::new(
            SingleExit::new::<Self>(&self),
            InternalPureSignatures::new::<Self>(&self),
            take!(input, FieldOrder),
            ResolvedSubstitutions::new::<Self>(&self),
        )
    }
}


pub struct PropagateExtern;
impl ProveFact<ExternStateInSignatures> for PropagateExtern {}

define_fact_set!(Kir1_2S1, [SingleExit, PureSignatures, FieldOrder, ResolvedSubstitutions]);

impl Process for PropagateExtern {
    type Requires = Kir1;
    type Provides = Kir1_2S1;

    fn run(self, input: Kir1) -> Kir1_2S1 {
        Kir1_2S1::new(
            take!(input, SingleExit),
            PureSignatures::new(
                take!(input, InternalPureSignatures),
                ExternStateInSignatures::new::<Self>(&self),
            ),
            take!(input, FieldOrder),
            take!(input, ResolvedSubstitutions),
        )
    }
}

pub struct StructCanonStep;
impl ProveFact<NoNumberedFields> for StructCanonStep {}

define_fact_set!(Kir1_2, [SingleExit, PureSignatures, StructCanon, ResolvedSubstitutions]);

impl Process for StructCanonStep {
    type Requires = Kir1_2S1;
    type Provides = Kir1_2;

    fn run(self, input: Self::Requires) -> Self::Provides {
        Kir1_2::new(
            take!(input, SingleExit),
            take!(input, PureSignatures),
            StructCanon::new(
                NoNumberedFields::new::<Self>(&self),
                take!(input, FieldOrder),
            ),
            take!(input, ResolvedSubstitutions),
            )
    }
}