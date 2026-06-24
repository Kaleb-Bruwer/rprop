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
        pub type #name = fn(#premise_ty) -> #conclusion_ty;

        // impl ::rprop::Sorry for #name {
        //     fn sorry() -> Self {
        //         |_premise: #premise_ty| #conclusion_ty::sorry()
        //     }
        // }
    }
}
