use rose_architecture_macros::propose;

use crate::{
    framework::{Process, ProvideProp, take},
    props::{
        ExternStateInSignatures, FieldOrder, InternalPureSignatures, NumberedFieldsRenamed, PureSignatures,
        ResolvedSubstitutions, SingleExit, StructCanon, ValidSourceProgram,
    },
};

pub struct GetAST;

impl ProvideProp<ValidSourceProgram> for GetAST {}
impl ProvideProp<FieldOrder> for GetAST {}

propose!(SourceAST = ValidSourceProgram && FieldOrder);

impl Process for GetAST {
    type Requires = ();
    type Provides = SourceAST;

    fn run(self, _input: ()) -> SourceAST {
        SourceAST::new(ValidSourceProgram::provide::<Self>(&self), FieldOrder::provide::<Self>(&self))
    }
}

pub struct KirBuilder;

impl ProvideProp<SingleExit> for KirBuilder {}
impl ProvideProp<InternalPureSignatures> for KirBuilder {}
impl ProvideProp<ResolvedSubstitutions> for KirBuilder {}

propose!(Kir1 = SingleExit && InternalPureSignatures && FieldOrder && ResolvedSubstitutions);

impl Process for KirBuilder {
    type Requires = SourceAST;
    type Provides = Kir1;

    fn run(self, input: SourceAST) -> Kir1 {
        Kir1::new(
            SingleExit::provide::<Self>(&self),
            InternalPureSignatures::provide::<Self>(&self),
            take!(input, FieldOrder),
            ResolvedSubstitutions::provide::<Self>(&self),
        )
    }
}

pub struct PropagateExtern;
impl ProvideProp<ExternStateInSignatures> for PropagateExtern {}

propose!(Kir1_2S1 = SingleExit && PureSignatures && FieldOrder && ResolvedSubstitutions);

impl Process for PropagateExtern {
    type Requires = Kir1;
    type Provides = Kir1_2S1;

    fn run(self, input: Kir1) -> Kir1_2S1 {
        Kir1_2S1::new(
            take!(input, SingleExit),
            PureSignatures::new(take!(input, InternalPureSignatures), ExternStateInSignatures::provide::<Self>(&self)),
            take!(input, FieldOrder),
            take!(input, ResolvedSubstitutions),
        )
    }
}

pub struct StructCanonStep;

propose!(Kir1_2 = SingleExit && PureSignatures && StructCanon && ResolvedSubstitutions);

impl Process for StructCanonStep {
    type Requires = Kir1_2S1;
    type Provides = Kir1_2;

    fn run(self, input: Self::Requires) -> Self::Provides {
        let renamed = NumberedFieldsRenamed::new(take!(input, FieldOrder));

        Kir1_2::new(
            take!(input, SingleExit),
            take!(input, PureSignatures),
            StructCanon::from(renamed),
            take!(input, ResolvedSubstitutions),
        )
    }
}
