use quote::quote;
use syn::{Attribute, Ident, Result};

use crate::{
    ast::NamedExpr,
    emit::members::{conclusion_ty, premise_params},
};

pub fn emit_implication(
    attrs: &[Attribute],
    name: &Ident,
    premise: &NamedExpr,
    conclusion: &NamedExpr,
) -> Result<proc_macro2::TokenStream> {
    let params = premise_params(premise)?;
    let return_ty = conclusion_ty(conclusion);
    let param_names: Vec<_> = params.iter().map(|(name, _)| name).collect();
    let param_tys: Vec<_> = params.iter().map(|(_, ty)| ty).collect();

    Ok(quote! {
        #(#attrs)*
        pub type #name = fn(#( #param_names: #param_tys ),*) -> #return_ty;
    })
}
