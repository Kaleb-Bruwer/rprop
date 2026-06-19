use std::collections::HashMap;

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    Attribute, Ident, Token,
};

struct FactSetInput {
    attrs: Vec<Attribute>,
    name: Ident,
    facts: Vec<FactEntry>,
}

enum FactEntry {
    Plain(Ident),
    Aliased { fact: Ident, field: Ident },
}

impl Parse for FactSetInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        let content;
        syn::bracketed!(content in input);

        let mut facts = Vec::new();
        while !content.is_empty() {
            let fact: Ident = content.parse()?;
            if content.peek(Token![as]) {
                content.parse::<Token![as]>()?;
                let field: Ident = content.parse()?;
                facts.push(FactEntry::Aliased { fact, field });
            } else {
                facts.push(FactEntry::Plain(fact));
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(FactSetInput { attrs, name, facts })
    }
}

struct ResolvedFact {
    fact: Ident,
    field: Ident,
}

fn resolve_facts(facts: Vec<FactEntry>) -> syn::Result<Vec<ResolvedFact>> {
    let mut seen_facts: HashMap<String, Ident> = HashMap::new();
    let mut seen_fields: HashMap<String, Ident> = HashMap::new();
    let mut resolved = Vec::new();

    for entry in facts {
        let (fact, field) = match entry {
            FactEntry::Plain(fact) => {
                let field_name = fact.to_string().to_snake_case();
                let field = format_ident!("{}", field_name, span = fact.span());
                (fact, field)
            }
            FactEntry::Aliased { fact, field } => (fact, field),
        };

        if let Some(other) = seen_facts.get(fact.to_string().as_str()) {
            return Err(syn::Error::new(
                fact.span(),
                format!("duplicate fact `{fact}` (already listed as `{other}`)"),
            ));
        }
        seen_facts.insert(fact.to_string(), fact.clone());

        let field_key = field.to_string();
        if let Some(other) = seen_fields.get(&field_key) {
            return Err(syn::Error::new(
                field.span(),
                format!(
                    "`{fact}` maps to field `{field_key}`, which is already used by `{other}`; \
                     use `{fact} as custom_name` to disambiguate"
                ),
            ));
        }
        seen_fields.insert(field_key, fact.clone());

        resolved.push(ResolvedFact { fact, field });
    }

    Ok(resolved)
}

fn error_tokens(message: impl AsRef<str>, span: Span) -> TokenStream {
    syn::Error::new(span, message.as_ref())
        .to_compile_error()
        .into()
}

#[proc_macro]
pub fn define_fact_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FactSetInput);

    if input.facts.is_empty() {
        return error_tokens(
            "define_fact_set requires at least one fact",
            input.name.span(),
        );
    }

    let resolved = match resolve_facts(input.facts) {
        Ok(facts) => facts,
        Err(error) => return error.to_compile_error().into(),
    };

    let name = &input.name;
    let attrs = &input.attrs;

    let field_defs = resolved.iter().map(|r| {
        let fact = &r.fact;
        let field = &r.field;
        quote! { pub #field: #fact }
    });
    let new_params = resolved.iter().map(|r| {
        let fact = &r.fact;
        let field = &r.field;
        quote! { #field: #fact }
    });
    let new_fields = resolved.iter().map(|r| &r.field);
    let has_fact_impls = resolved.iter().map(|r| {
        let fact = &r.fact;
        let field = &r.field;
        quote! {
            impl crate::framework::HasFact<#fact> for #name {
                fn fact(&self) -> #fact {
                    self.#field
                }
            }
        }
    });

    let expanded = quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name {
            #(#field_defs,)*
        }

        impl #name {
            pub fn new(#(#new_params,)*) -> Self {
                Self {
                    #(#new_fields,)*
                }
            }
        }

        #(#has_fact_impls)*
    };

    expanded.into()
}
