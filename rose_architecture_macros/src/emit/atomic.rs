use quote::quote;
use syn::{Attribute, Ident};

pub fn emit_atomic(attrs: &[Attribute], name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name {
            _private: (),
        }

        impl crate::framework::Prop for #name {}

        impl #name {
            pub(crate) fn provide<P: crate::framework::ProvideProp<Self>>(_provider: &P) -> Self {
                Self { _private: () }
            }
        }

        impl crate::framework::Sorry for #name {
            fn sorry() -> Self {
                Self { _private: () }
            }
        }
    }
}
