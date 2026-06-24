use quote::quote;
use syn::{Attribute, Error, Ident, Result};

use crate::ast::ProposeKind;

pub fn emit_disjunction(
    attrs: &[Attribute],
    name: &Ident,
    variants: &[Ident],
    kind: ProposeKind,
) -> Result<proc_macro2::TokenStream> {
    if variants.len() < 2 {
        return Err(Error::new_spanned(name, "disjunction requires at least two variants"));
    }

    let default_variant = &variants[0];

    Ok(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub enum #name {
            #( #variants(#variants) ),*
        }

        impl ::rprop::Prop for #name {}
        impl ::rprop::Disjunction for #name {}

        #(
            impl From<#variants> for #name {
                fn from(prop: #variants) -> Self {
                    Self::#variants(prop)
                }
            }
        )*

        // This should not be a long term solution
        impl ::rprop::Sorry for #name {
            fn sorry() -> Self {
                Self::#default_variant(<#default_variant as ::rprop::Sorry>::sorry())
            }
        }
    })
}
