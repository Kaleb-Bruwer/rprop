use quote::format_ident;
use syn::{Error, Ident, Result};

use crate::ast::{NamedExpr, NamedPropExpr, PropExpr, ProposeInput};

pub struct NameFactory {
    base: Ident,
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

pub fn lower_propose(input: ProposeInput) -> Result<NamedPropExpr> {
    match input.expr {
        None => Ok(NamedPropExpr {
            name: input.name.clone(),
            expr: NamedExpr::Atom,
        }),
        Some(expr) => {
            let mut factory = NameFactory::new(input.name.clone());
            name_expr(expr, Some(input.name), &mut factory)
        }
    }
}

fn name_expr(
    expr: PropExpr,
    user_name: Option<Ident>,
    factory: &mut NameFactory,
) -> Result<NamedPropExpr> {
    match expr {
        PropExpr::Atom(ident) => {
            if let Some(name) = user_name {
                Ok(NamedPropExpr {
                    name,
                    expr: NamedExpr::And(vec![Box::new(NamedPropExpr {
                        name: ident.clone(),
                        expr: NamedExpr::Atom,
                    })]),
                })
            } else {
                Ok(NamedPropExpr {
                    name: ident.clone(),
                    expr: NamedExpr::Atom,
                })
            }
        }
        PropExpr::And(children) => name_composite(children, user_name, factory, true),
        PropExpr::Or(children) => {
            if children.len() < 2 {
                return Err(Error::new_spanned(
                    &factory.base,
                    "disjunction requires at least two operands",
                ));
            }
            name_composite(children, user_name, factory, false)
        }
    }
}

fn name_composite(
    children: Vec<Box<PropExpr>>,
    user_name: Option<Ident>,
    factory: &mut NameFactory,
    is_and: bool,
) -> Result<NamedPropExpr> {
    let name = match user_name {
        Some(n) => n,
        None => factory.next(),
    };

    let named_children: Result<Vec<Box<NamedPropExpr>>> = children
        .into_iter()
        .map(|c| name_expr(*c, None, factory).map(Box::new))
        .collect();

    let expr = if is_and {
        NamedExpr::And(named_children?)
    } else {
        NamedExpr::Or(named_children?)
    };

    Ok(NamedPropExpr { name, expr })
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
        assert_eq!(named.name.to_string(), "X");
        let NamedExpr::And(children) = &named.expr else {
            panic!("expected And");
        };
        assert_eq!(children[1].name.to_string(), "X_0");
    }
}
