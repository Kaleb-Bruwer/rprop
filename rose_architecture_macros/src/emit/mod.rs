mod atomic;
mod conjunction;
mod disjunction;

pub use atomic::emit_atomic;
pub use conjunction::emit_conjunction;
pub use disjunction::emit_disjunction;

use quote::quote;
use syn::{Ident, Result};

use crate::ast::{NamedExpr, NamedPropExpr, ProposeInput};

pub fn emit_propose(input: ProposeInput, named: &NamedPropExpr) -> Result<proc_macro2::TokenStream> {
    let mut nodes = Vec::new();
    named.collect_postorder(&mut nodes);

    let mut emitted = Vec::new();
    let is_bare_atomic = input.expr.is_none();

    for node in nodes {
        match &node.expr {
            NamedExpr::Atom => {
                if is_bare_atomic && node.name == input.name {
                    emitted.push(emit_atomic(&input.attrs, &node.name));
                }
            }
            NamedExpr::And(children) => {
                let members: Vec<Ident> = children.iter().map(|c| c.name.clone()).collect();
                let attrs = if node.name == named.name {
                    input.attrs.clone()
                } else {
                    Vec::new()
                };
                emitted.push(emit_conjunction(&attrs, &node.name, &members)?);
            }
            NamedExpr::Or(children) => {
                let variants: Vec<Ident> = children.iter().map(|c| c.name.clone()).collect();
                let attrs = if node.name == named.name {
                    input.attrs.clone()
                } else {
                    Vec::new()
                };
                emitted.push(emit_disjunction(&attrs, &node.name, &variants)?);
            }
        }
    }

    Ok(quote! { #(#emitted)* })
}
