mod atomic;
mod conjunction;
mod disjunction;
mod implication;

pub use atomic::emit_atomic;
pub use conjunction::emit_conjunction;
pub use disjunction::emit_disjunction;
pub use implication::emit_implication;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

use crate::ast::{NamedExpr, ProposeInput, ProposeKind};

pub fn emit_propose(input: ProposeInput, named: &NamedExpr, kind: ProposeKind) -> Result<proc_macro2::TokenStream> {
    let mut emitted = Vec::new();
    if input.expr.is_none() {
        emitted.push(emit_atomic(&input.attrs, &input.name, kind));
    } else {
        emitted = expr_tokenstream(input, named, kind)?;
    }

    Ok(quote! { #(#emitted)* })
}

/// Only call for non-atomic propositions, i.e. expressions with a rhs
fn expr_tokenstream(input: ProposeInput, named: &NamedExpr, kind: ProposeKind) -> Result<Vec<TokenStream>> {
    let mut nodes = Vec::new();
    named.collect_postorder(&mut nodes);

    let mut emitted = Vec::new();
    for node in nodes {
        let is_root = node.name() == input.name;
        let node_kind = if is_root { kind } else { ProposeKind::Claim };
        match node {
            NamedExpr::And { name, children } => {
                let members: Vec<_> = children.iter().map(|c| c.name()).collect();
                let attrs = if is_root { input.attrs.clone() } else { Vec::new() };
                emitted.push(emit_conjunction(&attrs, name, &members, node_kind)?);
            }
            NamedExpr::Or { name, children } => {
                let variants: Vec<_> = children.iter().map(|c| c.name()).collect();
                let attrs = if is_root { input.attrs.clone() } else { Vec::new() };
                emitted.push(emit_disjunction(&attrs, name, &variants, node_kind)?);
            }
            NamedExpr::Imply { name, premise, conclusion } => {
                let attrs = if is_root { input.attrs.clone() } else { Vec::new() };
                emitted.push(emit_implication(&attrs, name, premise, conclusion, node_kind));
            }
            NamedExpr::Atom(_) => {}
        }
    }

    Ok(emitted)
}
