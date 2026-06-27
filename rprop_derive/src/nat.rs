//! Mirror of `rprop::Nat`. Keep in sync with `rprop/src/lib.rs`.

pub(crate) type Nat = u32;

/// Token stream for the user-facing Nat type in generated code.
pub(crate) fn emit_nat_ty() -> proc_macro2::TokenStream {
    quote::quote!(::rprop::Nat)
}

/// Parse a macro-input integer literal into a Nat value.
pub(crate) fn parse_lit(lit: &syn::LitInt) -> syn::Result<Nat> {
    lit.base10_parse::<Nat>()
}
