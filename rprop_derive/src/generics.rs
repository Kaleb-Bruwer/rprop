use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use std::collections::HashSet;

use crate::{
    ast::{Atom, NamedExpr, NatArg, ProposeInput},
    nat,
};

pub struct GenericContext {
    pub binders: Vec<Ident>,
}

impl GenericContext {
    pub fn from_input(input: &ProposeInput) -> Self {
        Self { binders: input.binders.clone() }
    }

    pub fn from_binders(binders: Vec<Ident>) -> Self {
        Self { binders }
    }

    pub fn has_binders(&self) -> bool {
        !self.binders.is_empty()
    }
}

pub fn emit_const_params(binders: &[Ident]) -> TokenStream {
    if binders.is_empty() {
        return TokenStream::new();
    }

    let nat = nat::emit_nat_ty();
    quote! { < #( const #binders: #nat ),* > }
}

pub fn emit_const_arg_list(binders: &[Ident]) -> TokenStream {
    if binders.is_empty() {
        return TokenStream::new();
    }

    quote! { < #(#binders),* > }
}

pub fn emit_obligation_args(binders: &[Ident]) -> TokenStream {
    if binders.is_empty() {
        return TokenStream::new();
    }

    // There's no such thing as initializing every possible generic, so 0 will have to do for now.
    let zeros = binders.iter().map(|_| {
        let lit = syn::LitInt::new("0", proc_macro2::Span::call_site());
        quote! { #lit }
    });
    quote! { < #(#zeros),* > }
}

impl Atom {
    pub fn base_name(&self) -> Ident {
        self.name.clone()
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name = &self.name;
        if self.args.is_empty() {
            quote! { #name }
        } else {
            let args = self.args.iter().map(NatArg::to_tokens);
            quote! { #name< #(#args),* > }
        }
    }

    pub fn field_suffix(&self) -> String {
        if self.args.is_empty() {
            return String::new();
        }

        self.args
            .iter()
            .map(|arg| match arg {
                NatArg::Lit(n) => n.to_string(),
                NatArg::Param(ident) => ident.to_string().to_snake_case(),
            })
            .collect::<Vec<_>>()
            .join("_")
    }

    pub fn field_name(&self) -> Ident {
        let base = self.name.to_string().to_snake_case();
        let suffix = self.field_suffix();
        let name = if suffix.is_empty() { base } else { format!("{base}_{suffix}") };
        format_ident!("{}", name, span = self.name.span())
    }
}

impl NatArg {
    pub fn to_tokens(&self) -> TokenStream {
        match self {
            NatArg::Lit(n) => {
                let lit = syn::LitInt::new(&n.to_string(), proc_macro2::Span::call_site());
                quote! { #lit }
            }
            NatArg::Param(ident) => quote! { #ident },
        }
    }
}

impl Atom {
    pub fn collect_param_binders_into(&self, binders: &mut Vec<Ident>, seen: &mut HashSet<String>) {
        for arg in &self.args {
            if let NatArg::Param(ident) = arg {
                if seen.insert(ident.to_string()) {
                    binders.push(ident.clone());
                }
            }
        }
    }
}

impl NamedExpr {
    pub fn collect_param_binders(&self) -> Vec<Ident> {
        let mut binders = Vec::new();
        let mut seen = HashSet::new();
        self.collect_param_binders_into(&mut binders, &mut seen);
        binders
    }

    fn collect_param_binders_into(&self, binders: &mut Vec<Ident>, seen: &mut HashSet<String>) {
        match self {
            NamedExpr::Atom(atom) => atom.collect_param_binders_into(binders, seen),
            NamedExpr::And { children, .. } | NamedExpr::Or { children, .. } => {
                for child in children {
                    child.collect_param_binders_into(binders, seen);
                }
            }
            NamedExpr::Imply { premise, conclusion, .. } => {
                premise.collect_param_binders_into(binders, seen);
                conclusion.collect_param_binders_into(binders, seen);
            }
        }
    }

    pub fn reference_tokens(&self) -> TokenStream {
        match self {
            NamedExpr::Atom(atom) => atom.to_tokens(),
            named => {
                let name = named.name();
                let args = emit_const_arg_list(&named.collect_param_binders());
                quote! { #name #args }
            }
        }
    }

    pub fn member_field_name(&self) -> Ident {
        match self {
            NamedExpr::Atom(atom) => atom.field_name(),
            named => {
                let base = named.name().to_string().to_snake_case();
                let suffix = named
                    .collect_param_binders()
                    .iter()
                    .map(|ident| ident.to_string().to_snake_case())
                    .collect::<Vec<_>>()
                    .join("_");
                let name = if suffix.is_empty() { base } else { format!("{base}_{suffix}") };
                format_ident!("{}", name, span = named.name().span())
            }
        }
    }
}
