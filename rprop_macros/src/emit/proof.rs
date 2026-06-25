use quote::{format_ident, quote};
use syn::Ident;

pub fn proof_trait_name(name: &Ident) -> Ident {
    format_ident!("__rprop_{}_proof", name)
}

pub fn emit_claim_obligation(name: &Ident) -> proc_macro2::TokenStream {
    let trait_name = proof_trait_name(name);
    let obligation_name = format_ident!("__rprop_{}_obligation", name);

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

pub fn emit_proof_binding(claim: &Ident, fn_name: &Ident) -> proc_macro2::TokenStream {
    let trait_name = proof_trait_name(claim);

    quote! {
        impl #trait_name for #claim {
            const PROOF: Self = #fn_name;
        }
    }
}
