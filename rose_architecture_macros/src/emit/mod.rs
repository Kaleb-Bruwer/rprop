mod atomic;
mod conjunction;
mod disjunction;

pub use atomic::emit_atomic;
pub use conjunction::emit_conjunction;
pub use disjunction::emit_disjunction;

use quote::quote;
use syn::Result;

use crate::ast::{NamedExpr, ProposeInput};

pub fn emit_propose(input: ProposeInput, named: &NamedExpr) -> Result<proc_macro2::TokenStream> {
    let mut nodes = Vec::new();
    named.collect_postorder(&mut nodes);

    let mut emitted = Vec::new();
    let is_bare_atomic = input.expr.is_none();

    for node in nodes {
        match node {
            NamedExpr::Atom(ident) => {
                if is_bare_atomic && *ident == input.name {
                    emitted.push(emit_atomic(&input.attrs, ident));
                }
            }
            NamedExpr::And { name, children } => {
                let members: Vec<_> = children.iter().map(|c| c.member_type()).collect();
                let attrs = if *name == input.name {
                    input.attrs.clone()
                } else {
                    Vec::new()
                };
                emitted.push(emit_conjunction(&attrs, name, &members)?);
            }
            NamedExpr::Or { name, children } => {
                let variants: Vec<_> = children.iter().map(|c| c.member_type()).collect();
                let attrs = if *name == input.name {
                    input.attrs.clone()
                } else {
                    Vec::new()
                };
                emitted.push(emit_disjunction(&attrs, name, &variants)?);
            }
        }
    }

    Ok(quote! { #(#emitted)* })
}
