use quote::quote;
use syn::{Attribute, Ident};

pub fn emit_atomic(attrs: &[Attribute], name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name {
            _private: (),
        }

        impl crate::framework::Fact for #name {}

        impl #name {
            pub(crate) fn new<P: crate::framework::ProveFact<Self>>(_provider: &P) -> Self {
                Self { _private: () }
            }
        }
    }
}
