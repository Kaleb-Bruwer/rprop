use quote::format_ident;
use syn::{Error, Ident, Result};

use crate::{keywords, lower::NameFactory};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropExpr {
    Atom(Ident),
    Not(Box<PropExpr>),
    And(Vec<Box<PropExpr>>),
    Or(Vec<Box<PropExpr>>),
    Imply(Box<PropExpr>, Box<PropExpr>),
}

#[derive(Debug, Clone)]
pub enum NamedExpr {
    Atom(Ident),
    And { name: Ident, children: Vec<Box<NamedExpr>> },
    Or { name: Ident, children: Vec<Box<NamedExpr>> },
    Imply { name: Ident, premise: Box<NamedExpr>, conclusion: Box<NamedExpr> },
}

#[derive(Clone)]
pub struct ProposeInput {
    pub items: Vec<ProposeItem>,
}

#[derive(Clone)]
pub struct ProposeItem {
    pub attrs: Vec<syn::Attribute>,
    pub name: Ident,
    pub expr: Option<PropExpr>,
}

impl NamedExpr {
    pub fn name(&self) -> Ident {
        match self {
            NamedExpr::Atom(ident) => ident.clone(),
            NamedExpr::And { name, .. } | NamedExpr::Or { name, .. } => name.clone(),
            NamedExpr::Imply { name, .. } => name.clone(),
        }
    }

    pub fn collect_postorder<'a>(&'a self, out: &mut Vec<&'a NamedExpr>) {
        match self {
            NamedExpr::Atom(_) => {}
            NamedExpr::And { children, .. } | NamedExpr::Or { children, .. } => {
                for child in children {
                    child.collect_postorder(out);
                }
            }
            NamedExpr::Imply { premise, conclusion, .. } => {
                premise.collect_postorder(out);
                conclusion.collect_postorder(out);
            }
        }
        out.push(self);
    }
}

impl PropExpr {
    pub fn imply_absurd(self) -> PropExpr {
        PropExpr::Imply(Box::new(self), Box::new(PropExpr::Atom(format_ident!("{}", keywords::ABSURD))))
    }

    pub fn into_named(self, factory: &mut NameFactory) -> Result<NamedExpr> {
        match self {
            PropExpr::Atom(ident) => Ok(NamedExpr::Atom(ident)),
            PropExpr::Not(inner) => (*inner).imply_absurd().into_named(factory),
            PropExpr::And(children) => {
                let named_children: Result<Vec<Box<NamedExpr>>> =
                    children.into_iter().map(|c| c.into_named(factory).map(Box::new)).collect();
                Ok(NamedExpr::And { name: factory.next(), children: named_children? })
            }
            PropExpr::Or(children) => {
                if children.len() < 2 {
                    return Err(Error::new_spanned(&factory.base, "disjunction requires at least two operands"));
                }
                let named_children: Result<Vec<Box<NamedExpr>>> =
                    children.into_iter().map(|c| c.into_named(factory).map(Box::new)).collect();
                Ok(NamedExpr::Or { name: factory.next(), children: named_children? })
            }
            PropExpr::Imply(premise, conclusion) => {
                let premise_named = premise.into_named(factory)?;
                let conclusion_named = conclusion.into_named(factory)?;
                Ok(NamedExpr::Imply {
                    name: factory.next(),
                    premise: Box::new(premise_named),
                    conclusion: Box::new(conclusion_named),
                })
            }
        }
    }
}
