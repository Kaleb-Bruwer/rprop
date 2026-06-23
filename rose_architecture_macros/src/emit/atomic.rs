use quote::quote;
use syn::{Attribute, Ident};

use crate::ast::ProposeKind;

pub fn emit_atomic(attrs: &[Attribute], name: &Ident, kind: ProposeKind) -> proc_macro2::TokenStream {
    let provide = match kind {
        ProposeKind::Proposition => quote! {
            impl #name {
                pub(crate) fn provide<P: crate::framework::ProvideProp<Self>>(_provider: &P) -> Self {
                    Self { _private: () }
                }
            }
        },
        ProposeKind::Claim => quote! {},
    };

    quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name {
            _private: (),
        }

        impl crate::framework::Prop for #name {}

        #provide

        impl crate::framework::Sorry for #name {
            fn sorry() -> Self {
                Self { _private: () }
            }
        }
    }
}
