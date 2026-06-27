use quote::quote;
use syn::Attribute;

use crate::generics::{emit_const_arg_list, emit_const_params, GenericContext};

pub fn emit_atomic(attrs: &[Attribute], name: &syn::Ident, ctx: &GenericContext) -> proc_macro2::TokenStream {
    let const_params = emit_const_params(&ctx.binders);
    let const_args = emit_const_arg_list(&ctx.binders);

    quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name #const_params {
            _private: (),
        }

        impl #const_params ::rprop::Prop for #name #const_args {}
    }
}
