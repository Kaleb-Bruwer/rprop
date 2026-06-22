use quote::format_ident;
use syn::{Error, Ident, Result};

use crate::ast::{NamedExpr, PropExpr, ProposeInput};

pub struct NameFactory {
    pub(crate) base: Ident,
    counter: u32,
}

impl NameFactory {
    pub fn new(base: Ident) -> Self {
        Self { base, counter: 0 }
    }

    pub fn next(&mut self) -> Ident {
        let ident = format_ident!("{}_{}", self.base, self.counter);
        self.counter += 1;
        ident
    }
}

pub fn lower_propose(input: ProposeInput) -> Result<NamedExpr> {
    match input.expr {
        None => Ok(NamedExpr::Atom(input.name)),
        Some(expr) => {
            let mut factory = NameFactory::new(input.name.clone());
            name_expr(expr, input.name, &mut factory)
        }
    }
}

fn name_expr(
    expr: PropExpr,
    user_name: Ident,
    factory: &mut NameFactory,
) -> Result<NamedExpr> {
    match expr {
        PropExpr::Atom(ident) => Ok(NamedExpr::And {
            name: user_name,
            children: vec![Box::new(NamedExpr::Atom(ident))],
        }),
        PropExpr::And(children) => {
            let named_children: Result<Vec<Box<NamedExpr>>> = children
                .into_iter()
                .map(|c| c.into_named(factory).map(Box::new))
                .collect();
            Ok(NamedExpr::And {
                name: user_name,
                children: named_children?,
            })
        }
        PropExpr::Or(children) => {
            if children.len() < 2 {
                return Err(Error::new_spanned(
                    &factory.base,
                    "disjunction requires at least two operands",
                ));
            }
            let named_children: Result<Vec<Box<NamedExpr>>> = children
                .into_iter()
                .map(|c| c.into_named(factory).map(Box::new))
                .collect();
            Ok(NamedExpr::Or {
                name: user_name,
                children: named_children?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;

    fn atom(name: &str) -> PropExpr {
        PropExpr::Atom(format_ident!("{}", name))
    }

    #[test]
    fn factory_yields_incrementing_names() {
        let mut factory = NameFactory::new(format_ident!("PureSignatures"));
        assert_eq!(factory.next().to_string(), "PureSignatures_0");
        assert_eq!(factory.next().to_string(), "PureSignatures_1");
    }

    #[test]
    fn nested_or_gets_factory_name() {
        let input = ProposeInput {
            attrs: vec![],
            name: format_ident!("X"),
            expr: Some(PropExpr::And(vec![
                Box::new(atom("A")),
                Box::new(PropExpr::Or(vec![Box::new(atom("B")), Box::new(atom("C"))])),
            ])),
        };
        let named = lower_propose(input).unwrap();
        let NamedExpr::And { name, children } = &named else {
            panic!("expected And");
        };
        assert_eq!(name.to_string(), "X");
        let NamedExpr::Or { name: or_name, .. } = &*children[1] else {
            panic!("expected Or");
        };
        assert_eq!(or_name.to_string(), "X_0");
    }
}
