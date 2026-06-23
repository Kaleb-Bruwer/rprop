use rose_architecture_macros::{claim, propose};

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
        SourceAST::new(
            <Self as ProvideProp<ValidSourceProgram>>::provide(&self),
            <Self as ProvideProp<FieldOrder>>::provide(&self),
        )
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
            <Self as ProvideProp<SingleExit>>::provide(&self),
            <Self as ProvideProp<InternalPureSignatures>>::provide(&self),
            take!(input, FieldOrder),
            <Self as ProvideProp<ResolvedSubstitutions>>::provide(&self),
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
            PureSignatures::new(
                take!(input, InternalPureSignatures),
                <Self as ProvideProp<ExternStateInSignatures>>::provide(&self),
            ),
            take!(input, FieldOrder),
            take!(input, ResolvedSubstitutions),
        )
    }
}

propose!(Kir1_2 = SingleExit && PureSignatures && StructCanon && ResolvedSubstitutions);
propose!(Renamed = FieldOrder -> NumberedFieldsRenamed);

claim!(StructCanonStep = Kir1_2S1 -> Kir1_2);

/// Constructive proof of `StructCanonStepI`: `Kir1_2S1 -> Kir1_2`.
fn struct_canon_proof(input: Kir1_2S1) -> Kir1_2 {
    let renamed = NumberedFieldsRenamed::new(take!(input, FieldOrder));

    Kir1_2::new(
        take!(input, SingleExit),
        take!(input, PureSignatures),
        StructCanon::from(renamed),
        take!(input, ResolvedSubstitutions),
    )
}

/// The proof function inhabits the claimed implication type.
pub const STRUCT_CANON_PROOF: StructCanonStep = struct_canon_proof;
