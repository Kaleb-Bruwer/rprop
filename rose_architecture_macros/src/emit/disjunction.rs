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

    let provide = match kind {
        ProposeKind::Proposition => quote! {
            impl #name {
                pub(crate) fn provide<P: crate::framework::ProvideProp<Self>>(_provider: &P) -> Self {
                    <Self as crate::framework::Sorry>::sorry()
                }
            }
        },
        ProposeKind::Claim => quote! {},
    };

    Ok(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub enum #name {
            #( #variants(#variants) ),*
        }

        impl crate::framework::Prop for #name {}
        impl crate::framework::Disjunction for #name {}

        #(
            impl From<#variants> for #name {
                fn from(prop: #variants) -> Self {
                    Self::#variants(prop)
                }
            }
        )*

        #provide

        // This should not be a long term solution
        impl crate::framework::Sorry for #name {
            fn sorry() -> Self {
                Self::#default_variant(<#default_variant as crate::framework::Sorry>::sorry())
            }
        }
    })
}
