mod atomic;
mod conjunction;
mod disjunction;

pub use atomic::emit_atomic;
pub use conjunction::emit_conjunction;
pub use disjunction::emit_disjunction;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

use crate::ast::{NamedExpr, ProposeInput};

pub fn emit_propose(input: ProposeInput, named: &NamedExpr) -> Result<proc_macro2::TokenStream> {
    let mut emitted = Vec::new();
    if input.expr.is_none() {
        // Bare atomic proposition, no rhs expression
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
        match node {
            NamedExpr::And { name, children } => {
                let members: Vec<_> = children.iter().map(|c| c.name()).collect();
                let attrs = if *name == input.name {
                    input.attrs.clone()
                } else {
                    Vec::new()
                };
                emitted.push(emit_conjunction(&attrs, name, &members)?);
            }
            NamedExpr::Or { name, children } => {
                let variants: Vec<_> = children.iter().map(|c| c.name()).collect();
                let attrs = if *name == input.name {
                    input.attrs.clone()
                } else {
                    Vec::new()
                };
                emitted.push(emit_disjunction(&attrs, name, &variants)?);
            }
            _ => {},
        }
    }

    Ok(emitted)
}