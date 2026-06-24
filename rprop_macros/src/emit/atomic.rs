use quote::quote;
use syn::{Attribute, Ident};

use crate::ast::ProposeKind;

pub fn emit_atomic(attrs: &[Attribute], name: &Ident, kind: ProposeKind) -> proc_macro2::TokenStream {
    quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name {
            _private: (),
        }

        impl ::rprop::Prop for #name {}

        impl ::rprop::Sorry for #name {
            fn sorry() -> Self {
                Self { _private: () }
            }
        }
    }
}
