use quote::quote;
use syn::{Attribute, Ident, Result, Error};

pub fn emit_disjunction(
    attrs: &[Attribute],
    name: &Ident,
    variants: &[Ident],
) -> Result<proc_macro2::TokenStream> {
    if variants.len() < 2 {
        return Err(Error::new_spanned(
            name,
            "disjunction requires at least two variants",
        ));
    }

    Ok(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub enum #name {
            #( #variants(#variants) ),*
        }

        impl crate::framework::Fact for #name {}
        impl crate::framework::Disjunction for #name {}

        #(
            impl From<#variants> for #name {
                fn from(fact: #variants) -> Self {
                    Self::#variants(fact)
                }
            }
        )*
    })
}
