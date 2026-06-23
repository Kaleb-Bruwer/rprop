use syn::{Error, Ident, Result};

use crate::lower::NameFactory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropExpr {
    Atom(Ident),
    And(Vec<Box<PropExpr>>),
    Or(Vec<Box<PropExpr>>),
}

#[derive(Debug, Clone)]
pub enum NamedExpr {
    Atom(Ident),
    And { name: Ident, children: Vec<Box<NamedExpr>> },
    Or { name: Ident, children: Vec<Box<NamedExpr>> },
}

#[derive(Clone)]
pub struct ProposeInput {
    pub attrs: Vec<syn::Attribute>,
    pub name: Ident,
    pub expr: Option<PropExpr>,
}

impl NamedExpr {
    pub fn name(&self) -> Ident {
        match self {
            NamedExpr::Atom(ident) => ident.clone(),
            NamedExpr::And { name, .. } | NamedExpr::Or { name, .. } => name.clone(),
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
        }
        out.push(self);
    }
}

impl PropExpr {
    pub fn into_named(self, factory: &mut NameFactory) -> Result<NamedExpr> {
        match self {
            PropExpr::Atom(ident) => Ok(NamedExpr::Atom(ident)),
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
        }
    }
}
