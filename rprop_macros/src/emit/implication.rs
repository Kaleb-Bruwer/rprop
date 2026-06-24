use quote::quote;
use syn::{Attribute, Ident};

use crate::ast::NamedExpr;

pub fn emit_implication(
    attrs: &[Attribute],
    name: &Ident,
    premise: &NamedExpr,
    conclusion: &NamedExpr,
) -> proc_macro2::TokenStream {
    let premise_ty = premise.name();
    let conclusion_ty = conclusion.name();

    quote! {
        #(#attrs)*
        pub type #name = fn(#premise_ty) -> #conclusion_ty;
    }
}
