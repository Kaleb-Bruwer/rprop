mod ast;
mod emit;
mod lower;
mod parse;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Ident, Result, Token,
};

use ast::ProposeInput;
use emit::{emit_claim, emit_conjunction, emit_disjunction, emit_propose};
use lower::lower_propose;

use crate::emit::emit_proof_binding;

struct BracketPropList {
    attrs: Vec<Attribute>,
    name: Ident,
    props: Vec<Ident>,
}

impl Parse for BracketPropList {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let content;
        syn::bracketed!(content in input);

        let mut props = Vec::new();
        while !content.is_empty() {
            props.push(content.parse()?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(BracketPropList { attrs, name, props })
    }
}

fn error_tokens(message: impl AsRef<str>, span: Span) -> TokenStream {
    syn::Error::new(span, message.as_ref()).to_compile_error().into()
}

fn emit_propose_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ProposeInput);

    let named = match lower_propose(input.clone()) {
        Ok(n) => n,
        Err(e) => return e.to_compile_error().into(),
    };

    match emit_propose(input, &named) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn propose(input: TokenStream) -> TokenStream {
    emit_propose_macro(input)
}

#[proc_macro]
pub fn claim(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ProposeInput);

    match emit_claim(input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn prove(args: TokenStream, input: TokenStream) -> TokenStream {
    let claim = parse_macro_input!(args as syn::Ident);
    let func = parse_macro_input!(input as syn::ItemFn);

    let binding = emit_proof_binding(&claim, &func.sig.ident);

    quote! {
        #func
        #binding
    }
    .into()
}

#[proc_macro]
pub fn define_conjunction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BracketPropList);

    if input.props.is_empty() {
        return error_tokens("define_conjunction requires at least one proposition", input.name.span());
    }

    match emit_conjunction(&input.attrs, &input.name, &input.props) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn define_disjunction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BracketPropList);

    if input.props.len() < 2 {
        return error_tokens("define_disjunction requires at least two propositions", input.name.span());
    }

    match emit_disjunction(&input.attrs, &input.name, &input.props) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(test)]
mod integration {
    use super::*;
    use ast::PropExpr;
    use lower::lower_propose;
    use quote::format_ident;
    use syn::parse_str;

    #[test]
    fn emit_nested_proposition() {
        let input = ProposeInput {
            attrs: vec![],
            name: format_ident!("PureSignatures"),
            expr: Some(PropExpr::And(vec![
                Box::new(PropExpr::Atom(format_ident!("InternalPureSignatures"))),
                Box::new(PropExpr::Or(vec![
                    Box::new(PropExpr::Atom(format_ident!("A"))),
                    Box::new(PropExpr::Atom(format_ident!("B"))),
                ])),
            ])),
        };
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("struct PureSignatures"));
        assert!(rendered.contains("enum PureSignatures0"));
    }

    #[test]
    fn parse_assign_input() {
        let input: ProposeInput = parse_str("StructCanon = NoNumberedFields || NumberedFieldsRenamed").unwrap();
        assert_eq!(input.name.to_string(), "StructCanon");
        assert!(matches!(input.expr, Some(PropExpr::Or(_))));
    }

    #[test]
    fn parse_implication_input() {
        let input: ProposeInput = parse_str("Renamed = FieldOrder -> NumberedFieldsRenamed").unwrap();
        assert_eq!(input.name.to_string(), "Renamed");
        assert!(matches!(input.expr, Some(PropExpr::Imply(_, _))));
    }

    #[test]
    fn claim_emits_obligation() {
        let input: ProposeInput = parse_str("TeaFromTap = A && B -> Tea").unwrap();
        let tokens = emit_claim(input).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("struct TeaFromTap"));
        assert!(rendered.contains("__rprop_TeaFromTap_obligation"));
        assert!(rendered.contains("__rprop_TeaFromTap_proof"));
    }

    #[test]
    fn claim_rejects_non_implication() {
        let input: ProposeInput = parse_str("NotAClaim = A && B").unwrap();
        let err = emit_claim(input).unwrap_err();
        assert!(err.to_string().contains("implication"));
    }
}
