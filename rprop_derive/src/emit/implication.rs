use quote::quote;
use syn::{Attribute, Ident, Result};

use crate::{
    ast::NamedExpr, emit::members::{conclusion_ty, premise_params}, keywords,
};

fn conclusion_return_ty(conclusion: &NamedExpr) -> proc_macro2::TokenStream {
    match conclusion {
        NamedExpr::Atom(id) if id == keywords::ABSURD => quote! { ::rprop::Absurd },
        _ => conclusion_ty(conclusion),
    }
}

pub fn emit_implication(
    attrs: &[Attribute],
    name: &Ident,
    premise: &NamedExpr,
    conclusion: &NamedExpr,
) -> Result<proc_macro2::TokenStream> {
    let params = premise_params(premise)?;
    let return_ty = conclusion_return_ty(conclusion);
    let param_names: Vec<_> = params.iter().map(|(name, _)| name).collect();
    let param_tys: Vec<_> = params.iter().map(|(_, ty)| ty).collect();

    Ok(quote! {
        #(#attrs)*
        pub type #name = fn(#( #param_names: #param_tys ),*) -> #return_ty;
    })
}
