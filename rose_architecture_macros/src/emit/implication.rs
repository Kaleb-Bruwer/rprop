use quote::quote;
use syn::{Attribute, Ident};

use crate::ast::{NamedExpr, ProposeKind};

pub fn emit_implication(
    attrs: &[Attribute],
    name: &Ident,
    premise: &NamedExpr,
    conclusion: &NamedExpr,
    kind: ProposeKind,
) -> proc_macro2::TokenStream {
    let premise_ty = premise.name();
    let conclusion_ty = conclusion.name();

    quote! {
        #(#attrs)*
        // pub type #name = crate::framework::Implies<#premise_ty, #conclusion_ty>;
        pub type #name = fn(#premise_ty) -> #conclusion_ty;

        // impl crate::framework::Sorry for #name {
        //     fn sorry() -> Self {
        //         |_premise: #premise_ty| #conclusion_ty::sorry()
        //     }
        // }
    }
}
