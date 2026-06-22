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
            Ok(ProposeInput {
                attrs,
                name,
                expr: Some(expr),
            })
        } else {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
            Ok(ProposeInput {
                attrs,
                name,
                expr: None,
            })
        }
    }
}

fn parse_expr(input: ParseStream) -> Result<PropExpr> {
    parse_or(input)
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
    let mut nodes = vec![parse_primary(input)?];
    while input.peek(Token![&&]) {
        input.parse::<Token![&&]>()?;
        nodes.push(parse_primary(input)?);
    }
    flatten_and(nodes)
}

fn parse_primary(input: ParseStream) -> Result<PropExpr> {
    if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        return parse_or(&content);
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
        parse_str::<ProposeInput>(&format!("X = {s}"))
            .expect("parse")
            .expr
            .expect("expr")
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
}
