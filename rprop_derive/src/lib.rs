mod ast;
mod emit;
mod generics;
mod keywords;
mod lower;
mod nat;
mod parse;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Ident, Result, Token,
};

use ast::{NamedExpr, ProposeInput};
use emit::{emit_claim, emit_conjunction, emit_disjunction, emit_propose};
use generics::GenericContext;
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

    let binding = emit_proof_binding(&claim, &func);

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

    let members: Vec<NamedExpr> =
        input.props.iter().map(|p| NamedExpr::Atom(ast::Atom::from_name(p.clone()))).collect();
    let member_refs: Vec<&NamedExpr> = members.iter().collect();

    match emit_conjunction(&input.attrs, &input.name, &member_refs, &GenericContext { binders: vec![] }) {
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

    let variants: Vec<NamedExpr> =
        input.props.iter().map(|p| NamedExpr::Atom(ast::Atom::from_name(p.clone()))).collect();
    let variant_refs: Vec<&NamedExpr> = variants.iter().collect();

    match emit_disjunction(&input.attrs, &input.name, &variant_refs, &GenericContext { binders: vec![] }) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[cfg(test)]
mod integration {
    use super::*;
    use ast::{Atom, PropExpr};
    use lower::lower_propose;
    use quote::format_ident;
    use syn::parse_str;

    fn atom(name: &str) -> PropExpr {
        PropExpr::Atom(Atom::from_name(format_ident!("{}", name)))
    }

    #[test]
    fn emit_nested_proposition() {
        let input = ProposeInput {
            attrs: vec![],
            name: format_ident!("PureSignatures"),
            binders: vec![],
            expr: Some(PropExpr::And(vec![
                Box::new(atom("InternalPureSignatures")),
                Box::new(PropExpr::Or(vec![Box::new(atom("A")), Box::new(atom("B"))])),
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
        assert!(!rendered.contains("struct TeaFromTap0"));
        assert!(rendered.contains("type TeaFromTap = fn"));
        assert!(rendered.contains("a : A"));
        assert!(rendered.contains("b : B"));
        assert!(rendered.contains("-> Tea"));
        assert!(rendered.contains("__rprop_TeaFromTap_obligation"));
        assert!(rendered.contains("__rprop_TeaFromTap_proof"));
    }

    #[test]
    fn implication_flattens_conjunction_premise_and_conclusion() {
        let input: ProposeInput = parse_str("Both = A && B -> C && D").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(!rendered.contains("struct Both0"));
        assert!(!rendered.contains("struct Both1"));
        assert!(rendered.contains("type Both = fn"));
        assert!(rendered.contains("a : A"));
        assert!(rendered.contains("b : B"));
        assert!(rendered.contains("-> (C , D)"));
    }

    #[test]
    fn nested_implication_flattens_operands() {
        let input: ProposeInput = parse_str("BoilWaterEnclosed = Water && Heat -> PressurizedSteam").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(!rendered.contains("struct BoilWaterEnclosed0"));
        assert!(rendered.contains("type BoilWaterEnclosed = fn"));
        assert!(rendered.contains("water : Water"));
        assert!(rendered.contains("heat : Heat"));
        assert!(rendered.contains("-> PressurizedSteam"));
    }

    #[test]
    fn standalone_conjunction_still_emits_struct() {
        let input: ProposeInput = parse_str("BoiledWater = Kettle && HasWater").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("struct BoiledWater"));
        assert!(rendered.contains("kettle : Kettle"));
        assert!(rendered.contains("has_water : HasWater"));
    }

    #[test]
    fn atomic_implication_unchanged() {
        let input: ProposeInput = parse_str("Renamed = FieldOrder -> NumberedFieldsRenamed").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("type Renamed = fn (field_order : FieldOrder) -> NumberedFieldsRenamed"));
    }

    #[test]
    fn claim_rejects_non_implication() {
        let input: ProposeInput = parse_str("NotAClaim = A && B").unwrap();
        let err = emit_claim(input).unwrap_err();
        assert!(err.to_string().contains("implication"));
    }

    #[test]
    fn negation_emits_implication_to_absurd() {
        let input: ProposeInput = parse_str("NotHot = !Hot").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("type NotHot = fn"));
        assert!(rendered.contains("hot : Hot"));
        assert!(rendered.contains(":: rprop :: Absurd"));
    }

    #[test]
    fn double_negation_emits_nested_implication() {
        let input: ProposeInput = parse_str("NotNotHot = !!Hot").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("type NotNotHot0 = fn"));
        assert!(rendered.contains("hot : Hot"));
        assert!(rendered.contains(":: rprop :: Absurd"));
        assert!(rendered.contains("type NotNotHot = fn"));
        assert!(rendered.contains("not_not_hot0 : NotNotHot0"));
    }

    #[test]
    fn contradiction_with_negation_in_premise() {
        let input: ProposeInput = parse_str("Contradiction = Hot && !Hot -> Absurd").unwrap();
        let tokens = emit_claim(input).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("type Contradiction = fn"));
        assert!(rendered.contains("hot : Hot"));
        assert!(rendered.contains(":: rprop :: Absurd"));
    }

    #[test]
    fn parse_generic_propose() {
        let input: ProposeInput = parse_str("At<N>").unwrap();
        assert_eq!(input.name.to_string(), "At");
        assert_eq!(input.binders.len(), 1);
        assert!(input.expr.is_none());
    }

    #[test]
    fn emit_generic_atomic() {
        let input: ProposeInput = parse_str("At<N>").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("struct At"));
        assert!(rendered.contains("const N :"));
        assert!(rendered.contains(":: rprop :: Nat"));
        assert!(!rendered.contains("PhantomData"));
    }

    #[test]
    fn emit_generic_conjunction_field_names() {
        let input: ProposeInput = parse_str("Pair<N, M> = At<N> && At<M>").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("at_n : At < N >"));
        assert!(rendered.contains("at_m : At < M >"));
    }

    #[test]
    fn emit_generic_claim() {
        let input: ProposeInput = parse_str("Refl<N> = At<N> -> Eq<N, N>").unwrap();
        let tokens = emit_claim(input).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("type Refl"));
        assert!(rendered.contains("const N :"));
        assert!(rendered.contains("at : At < N >"));
        assert!(rendered.contains("__rprop_Refl_proof"));
        assert!(rendered.contains("__rprop_Refl_obligation"));
    }

    #[test]
    fn emit_concrete_nat_literal() {
        let input: ProposeInput = parse_str("AtThree = At<3>").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        assert!(rendered.contains("at_3 : At < 3 >"));
    }

    #[test]
    fn emit_nested_generic_intermediate() {
        let input: ProposeInput = parse_str("X = A<N> && (B<M> || C<P>)").unwrap();
        let named = lower_propose(input.clone()).unwrap();
        let tokens = emit_propose(input, &named).unwrap();
        let rendered = tokens.to_string();
        println!("{rendered}");
        assert!(rendered.contains("enum X0"));
        assert!(rendered.contains("const M :"));
        assert!(rendered.contains("const P :"));
        assert!(rendered.contains("struct X"));
        assert!(rendered.contains("const N :"));
        assert!(rendered.contains("a_n : A < N >"));
        assert!(rendered.contains("x0_m_p : X0"));
    }
}
