use quote::{format_ident, quote};
use syn::{GenericParam, Ident, ItemFn};

use crate::generics::{emit_const_arg_list, emit_const_params, emit_obligation_args, GenericContext};

pub fn proof_trait_name(name: &Ident) -> Ident {
    format_ident!("__rprop_{}_proof", name)
}

pub fn emit_claim_obligation(name: &Ident, ctx: &GenericContext) -> proc_macro2::TokenStream {
    let trait_name = proof_trait_name(name);
    let obligation_name = format_ident!("__rprop_{}_obligation", name);
    let const_params = emit_const_params(&ctx.binders);

    if ctx.has_binders() {
        let const_args = emit_const_arg_list(&ctx.binders);
        let obligation_args = emit_obligation_args(&ctx.binders);
        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            pub trait #trait_name #const_params {
                const PROOF: #name #const_args;
            }

            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            const #obligation_name: #name #obligation_args = <#name #obligation_args as #trait_name #obligation_args>::PROOF;
        }
    } else {
        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            pub trait #trait_name {
                const PROOF: Self;
            }

            #[doc(hidden)]
            #[allow(non_upper_case_globals)]
            const #obligation_name: #name = <#name as #trait_name>::PROOF;
        }
    }
}

pub fn emit_proof_binding(claim: &Ident, func: &ItemFn) -> proc_macro2::TokenStream {
    let fn_name = &func.sig.ident;
    let trait_name = proof_trait_name(claim);

    let const_binders: Vec<Ident> = func
        .sig
        .generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Const(c) => Some(c.ident.clone()),
            _ => None,
        })
        .collect();

    if const_binders.is_empty() {
        quote! {
            impl #trait_name for #claim {
                const PROOF: Self = #fn_name;
            }
        }
    } else {
        let ctx = GenericContext { binders: const_binders };
        let const_params = emit_const_params(&ctx.binders);
        let const_args = emit_const_arg_list(&ctx.binders);

        quote! {
            impl #const_params #trait_name #const_args for #claim #const_args {
                const PROOF: Self = #fn_name;
            }
        }
    }
}
