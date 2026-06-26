use syn::{
    parse::{Parse, ParseStream},
    Attribute, Ident, Result, Token,
};

use crate::ast::{PropExpr, ProposeInput};

impl Parse for ProposeInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let name: Ident = input.parse()?;

        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let expr = parse_expr(input)?;
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            Ok(ProposeInput { attrs, name, expr: Some(expr) })
        } else {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            Ok(ProposeInput { attrs, name, expr: None })
        }
    }
}

fn parse_expr(input: ParseStream) -> Result<PropExpr> {
    parse_implies(input)
}

fn parse_implies(input: ParseStream) -> Result<PropExpr> {
    let mut lhs = parse_or(input)?;
    while input.peek(Token![->]) {
        input.parse::<Token![->]>()?;
        let rhs = parse_implies(input)?;
        lhs = PropExpr::Imply(Box::new(lhs), Box::new(rhs));
    }
    Ok(lhs)
}

fn parse_or(input: ParseStream) -> Result<PropExpr> {
    let mut nodes = vec![parse_and(input)?];
    while input.peek(Token![||]) {
        input.parse::<Token![||]>()?;
        nodes.push(parse_and(input)?);
    }
    flatten_or(nodes)
}

fn parse_and(input: ParseStream) -> Result<PropExpr> {
    let mut nodes = vec![parse_unary(input)?];
    while input.peek(Token![&&]) {
        input.parse::<Token![&&]>()?;
        nodes.push(parse_unary(input)?);
    }
    flatten_and(nodes)
}

fn parse_unary(input: ParseStream) -> Result<PropExpr> {
    if input.peek(Token![!]) {
        input.parse::<Token![!]>()?;
        let operand = parse_unary(input)?;
        return Ok(PropExpr::Not(Box::new(operand)));
    }
    parse_primary(input)
}

fn parse_primary(input: ParseStream) -> Result<PropExpr> {
    if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        return parse_expr(&content);
    }

    let ident: Ident = input.parse()?;
    Ok(PropExpr::Atom(ident))
}

fn flatten_and(nodes: Vec<PropExpr>) -> Result<PropExpr> {
    let mut flat: Vec<Box<PropExpr>> = Vec::new();
    for node in nodes {
        match node {
            PropExpr::And(children) => flat.extend(children),
            other => flat.push(Box::new(other)),
        }
    }
    if flat.len() == 1 {
        Ok(*flat.into_iter().next().unwrap())
    } else {
        Ok(PropExpr::And(flat))
    }
}

fn flatten_or(nodes: Vec<PropExpr>) -> Result<PropExpr> {
    let mut flat: Vec<Box<PropExpr>> = Vec::new();
    for node in nodes {
        match node {
            PropExpr::Or(children) => flat.extend(children),
            other => flat.push(Box::new(other)),
        }
    }
    if flat.len() == 1 {
        Ok(*flat.into_iter().next().unwrap())
    } else {
        Ok(PropExpr::Or(flat))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    fn parse_input(s: &str) -> ProposeInput {
        parse_str(s).expect("parse")
    }

    fn parse_only_expr(s: &str) -> PropExpr {
        parse_str::<ProposeInput>(&format!("X = {s}")).expect("parse").expr.expect("expr")
    }

    #[test]
    fn precedence_and_over_or() {
        let PropExpr::Or(or) = parse_only_expr("A || B && C") else {
            panic!("expected or");
        };
        assert!(matches!(&*or[0], PropExpr::Atom(_)));
        assert!(matches!(&*or[1], PropExpr::And(_)));
    }

    #[test]
    fn parentheses() {
        let PropExpr::And(and) = parse_only_expr("(A || B) && C") else {
            panic!("expected and");
        };
        assert!(matches!(&*and[0], PropExpr::Or(_)));
        assert!(matches!(&*and[1], PropExpr::Atom(_)));
    }

    #[test]
    fn bare_atomic() {
        let input = parse_input("ValidSourceProgram");
        assert!(input.expr.is_none());
    }

    #[test]
    fn named_conjunction() {
        let input = parse_input("PureSignatures = A && B");
        assert!(input.expr.is_some());
    }

    #[test]
    fn implies_over_and() {
        let PropExpr::Imply(premise, conclusion) = parse_only_expr("A && B -> C") else {
            panic!("expected imply");
        };
        assert!(matches!(&*premise, PropExpr::And(_)));
        assert!(matches!(&*conclusion, PropExpr::Atom(_)));
    }

    #[test]
    fn implies_right_associative() {
        let PropExpr::Imply(a, bc) = parse_only_expr("A -> B -> C") else {
            panic!("expected imply");
        };
        assert!(matches!(&*a, PropExpr::Atom(_)));
        let PropExpr::Imply(b, c) = &*bc else {
            panic!("expected nested imply");
        };
        assert!(matches!(&**b, PropExpr::Atom(_)));
        assert!(matches!(&**c, PropExpr::Atom(_)));
    }

    #[test]
    fn implies_in_parentheses_with_and() {
        let PropExpr::And(and) = parse_only_expr("(A -> B) && C") else {
            panic!("expected and");
        };
        assert!(matches!(&*and[0], PropExpr::Imply(_, _)));
        assert!(matches!(&*and[1], PropExpr::Atom(_)));
    }

    #[test]
    fn parse_negation() {
        let PropExpr::Not(inner) = parse_only_expr("!A") else {
            panic!("expected not");
        };
        assert!(matches!(&*inner, PropExpr::Atom(_)));
    }

    #[test]
    fn parse_double_negation() {
        let PropExpr::Not(inner) = parse_only_expr("!!A") else {
            panic!("expected not");
        };
        assert!(matches!(&*inner, PropExpr::Not(_)));
    }

    #[test]
    fn precedence_not_over_and() {
        let PropExpr::And(and) = parse_only_expr("!A && B") else {
            panic!("expected and");
        };
        assert!(matches!(&*and[0], PropExpr::Not(_)));
        assert!(matches!(&*and[1], PropExpr::Atom(_)));
    }

    #[test]
    fn precedence_not_over_or() {
        let PropExpr::Or(or) = parse_only_expr("!A || B") else {
            panic!("expected or");
        };
        assert!(matches!(&*or[0], PropExpr::Not(_)));
        assert!(matches!(&*or[1], PropExpr::Atom(_)));
    }

    #[test]
    fn precedence_not_in_implication_lhs() {
        let PropExpr::Imply(premise, conclusion) = parse_only_expr("!A -> B") else {
            panic!("expected imply");
        };
        assert!(matches!(&*premise, PropExpr::Not(_)));
        assert!(matches!(&*conclusion, PropExpr::Atom(_)));
    }

    #[test]
    fn precedence_not_in_implication_rhs() {
        let PropExpr::Imply(premise, conclusion) = parse_only_expr("A -> !B") else {
            panic!("expected imply");
        };
        assert!(matches!(&*premise, PropExpr::Atom(_)));
        assert!(matches!(&*conclusion, PropExpr::Not(_)));
    }

    #[test]
    fn negation_over_parenthesized_or() {
        let PropExpr::Not(inner) = parse_only_expr("!(A || B)") else {
            panic!("expected not");
        };
        assert!(matches!(&*inner, PropExpr::Or(_)));
    }
}
