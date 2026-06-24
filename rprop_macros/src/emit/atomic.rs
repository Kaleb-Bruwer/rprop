use quote::quote;
use syn::{Attribute, Ident};

pub fn emit_atomic(attrs: &[Attribute], name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name {
            _private: (),
        }

        impl ::rprop::Prop for #name {}
    }
}
