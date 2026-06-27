use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Result};

use crate::{
    ast::NamedExpr,
    emit::members::{conclusion_ty, premise_params},
    generics::{emit_const_params, GenericContext},
    keywords,
};

fn conclusion_return_ty(conclusion: &NamedExpr) -> TokenStream {
    match conclusion {
        NamedExpr::Atom(atom) if atom.base_name() == keywords::ABSURD => quote! { ::rprop::Absurd },
        _ => conclusion_ty(conclusion),
    }
}

pub fn emit_implication(
    attrs: &[Attribute],
    name: &Ident,
    premise: &NamedExpr,
    conclusion: &NamedExpr,
    ctx: &GenericContext,
) -> Result<proc_macro2::TokenStream> {
    let params = premise_params(premise)?;
    let return_ty = conclusion_return_ty(conclusion);
    let const_params = emit_const_params(&ctx.binders);
    let param_names: Vec<_> = params.iter().map(|(name, _)| name).collect();
    let param_tys: Vec<_> = params.iter().map(|(_, ty)| ty).collect();

    Ok(quote! {
        #(#attrs)*
        pub type #name #const_params = fn(#( #param_names: #param_tys ),*) -> #return_ty;
    })
}
