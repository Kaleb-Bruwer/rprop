use quote::quote;
use syn::{Attribute, Error, Ident, Result};

use crate::{
    ast::NamedExpr,
    generics::{emit_const_arg_list, emit_const_params, GenericContext},
};

pub fn emit_disjunction(
    attrs: &[Attribute],
    name: &Ident,
    variants: &[&NamedExpr],
    ctx: &GenericContext,
) -> Result<proc_macro2::TokenStream> {
    if variants.len() < 2 {
        return Err(Error::new_spanned(name, "disjunction requires at least two variants"));
    }

    let const_params = emit_const_params(&ctx.binders);
    let const_args = emit_const_arg_list(&ctx.binders);
    let variant_names: Vec<_> = variants.iter().map(|v| v.name()).collect();
    let variant_tys: Vec<_> = variants.iter().map(|v| v.reference_tokens()).collect();

    Ok(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub enum #name #const_params {
            #( #variant_names(#variant_tys) ),*
        }

        impl #const_params ::rprop::Prop for #name #const_args {}
        impl #const_params ::rprop::Disjunction for #name #const_args {}

        #(
            impl #const_params From<#variant_tys> for #name #const_args {
                fn from(prop: #variant_tys) -> Self {
                    Self::#variant_names(prop)
                }
            }
        )*
    })
}
