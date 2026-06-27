use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Result};

use crate::{ast::NamedExpr, emit::conjunction::resolve_conjunction_members};

pub fn premise_params(premise: &NamedExpr) -> Result<Vec<(Ident, TokenStream)>> {
    match premise {
        NamedExpr::And { children, .. } => {
            let members: Vec<&NamedExpr> = children.iter().map(|c| c.as_ref()).collect();
            Ok(resolve_conjunction_members(&members)?.into_iter().map(|m| (m.field, m.ty)).collect())
        }
        _ => {
            let (name, ty) = single_param(premise);
            Ok(vec![(name, ty)])
        }
    }
}

pub fn conclusion_ty(conclusion: &NamedExpr) -> TokenStream {
    match conclusion {
        NamedExpr::And { children, .. } => {
            let tys: Vec<TokenStream> = children.iter().map(|c| c.reference_tokens()).collect();
            match tys.len() {
                0 => unreachable!("conjunction requires at least one member"),
                1 => {
                    let ty = &tys[0];
                    quote! { (#ty,) }
                }
                _ => quote! { (#(#tys),*) },
            }
        }
        _ => conclusion.reference_tokens(),
    }
}

pub fn is_implication_and_operand(and_name: &Ident, root: &NamedExpr) -> bool {
    fn walk(expr: &NamedExpr, target: &Ident) -> bool {
        match expr {
            NamedExpr::Imply { premise, conclusion, .. } => {
                is_direct_and_operand(premise, target)
                    || is_direct_and_operand(conclusion, target)
                    || walk(premise, target)
                    || walk(conclusion, target)
            }
            NamedExpr::And { children, .. } | NamedExpr::Or { children, .. } => {
                children.iter().any(|c| walk(c, target))
            }
            NamedExpr::Atom(_) => false,
        }
    }

    fn is_direct_and_operand(expr: &NamedExpr, target: &Ident) -> bool {
        matches!(expr, NamedExpr::And { name, .. } if name == target)
    }

    walk(root, and_name)
}

fn single_param(expr: &NamedExpr) -> (Ident, TokenStream) {
    let name = format_ident!("{}", expr.name().to_string().to_snake_case(), span = expr.name().span());
    (name, expr.reference_tokens())
}
