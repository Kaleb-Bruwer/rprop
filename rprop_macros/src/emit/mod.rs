mod atomic;
mod conjunction;
mod disjunction;
mod implication;

pub use atomic::emit_atomic;
pub use conjunction::emit_conjunction;
pub use disjunction::emit_disjunction;
pub use implication::emit_implication;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Result};

use crate::ast::{NamedExpr, PropExpr, ProposeInput};

pub fn emit_propose(input: ProposeInput, named: &NamedExpr) -> Result<proc_macro2::TokenStream> {
    let mut emitted = Vec::new();
    if input.expr.is_none() {
        emitted.push(emit_atomic(&input.attrs, &input.name));
    } else {
        emitted = expr_tokenstream(input, named)?;
    }

    Ok(quote! { #(#emitted)* })
}

/// Only call for non-atomic propositions, i.e. expressions with a rhs
fn expr_tokenstream(input: ProposeInput, named: &NamedExpr) -> Result<Vec<TokenStream>> {
    let mut nodes = Vec::new();
    named.collect_postorder(&mut nodes);

    let mut emitted = Vec::new();
    for node in nodes {
        let is_root = node.name() == input.name;
        match node {
            NamedExpr::And { name, children } => {
                let members: Vec<_> = children.iter().map(|c| c.name()).collect();
                let attrs = if is_root { input.attrs.clone() } else { Vec::new() };
                emitted.push(emit_conjunction(&attrs, name, &members)?);
            }
            NamedExpr::Or { name, children } => {
                let variants: Vec<_> = children.iter().map(|c| c.name()).collect();
                let attrs = if is_root { input.attrs.clone() } else { Vec::new() };
                emitted.push(emit_disjunction(&attrs, name, &variants)?);
            }
            NamedExpr::Imply { name, premise, conclusion } => {
                let attrs = if is_root { input.attrs.clone() } else { Vec::new() };
                emitted.push(emit_implication(&attrs, name, premise, conclusion));
            }
            NamedExpr::Atom(_) => {}
        }
    }

    Ok(emitted)
}

pub fn emit_claim(input: ProposeInput) -> Result<proc_macro2::TokenStream> {
    let Some(expr) = &input.expr else {
        return Err(Error::new_spanned(&input.name, "claim requires an implication (`Premise -> Conclusion`)"));
    };

    if !matches!(expr, PropExpr::Imply(_, _)) {
        return Err(Error::new_spanned(&input.name, "claim requires an implication (`Premise -> Conclusion`)"));
    }

    let named = crate::lower::lower_propose(input.clone())?;
    let proposition = emit_propose(input.clone(), &named)?;
    let obligation = emit_claim_obligation(&input.name);

    Ok(quote! {
        #proposition
        #obligation
    })
}

fn emit_claim_obligation(name: &syn::Ident) -> proc_macro2::TokenStream {
    let trait_name = format_ident!("__rprop_{}_proof", name);
    let obligation_name = format_ident!("__rprop_{}_obligation", name);

    quote! {
        pub trait #trait_name {
            const PROOF: Self;
        }

        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        const #obligation_name: #name = <#name as #trait_name>::PROOF;
    }
}
