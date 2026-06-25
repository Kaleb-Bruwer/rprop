use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Result};

use crate::{
    ast::NamedExpr,
    emit::conjunction::resolve_conjunction_members,
};

pub fn premise_params(premise: &NamedExpr) -> Result<Vec<(Ident, Ident)>> {
    match premise {
        NamedExpr::And { children, .. } => {
            let members: Vec<Ident> = children.iter().map(|c| c.name()).collect();
            Ok(resolve_conjunction_members(&members)?
                .into_iter()
                .map(|m| (m.field, m.ty))
                .collect())
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
            let tys: Vec<Ident> = children.iter().map(|c| c.name()).collect();
            match tys.len() {
                0 => unreachable!("conjunction requires at least one member"),
                1 => {
                    let ty = &tys[0];
                    quote! { (#ty,) }
                }
                _ => quote! { (#(#tys),*) },
            }
        }
        _ => {
            let ty = conclusion.name();
            quote! { #ty }
        }
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

fn single_param(expr: &NamedExpr) -> (Ident, Ident) {
    let ty = expr.name();
    let name = format_ident!("{}", ty.to_string().to_snake_case(), span = ty.span());
    (name, ty)
}
