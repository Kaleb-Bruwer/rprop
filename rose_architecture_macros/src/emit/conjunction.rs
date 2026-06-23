use std::collections::HashMap;

use heck::ToSnakeCase;
use quote::{format_ident, quote};
use syn::{Attribute, Error, Ident, Result};

use crate::ast::ProposeKind;

pub struct ConjunctionMember {
    pub ty: Ident,
    pub field: Ident,
}

pub fn resolve_conjunction_members(members: &[Ident]) -> Result<Vec<ConjunctionMember>> {
    let mut seen_types: HashMap<String, Ident> = HashMap::new();
    let mut seen_fields: HashMap<String, Ident> = HashMap::new();
    let mut resolved = Vec::new();

    for ty in members {
        if let Some(other) = seen_types.get(&ty.to_string()) {
            return Err(Error::new_spanned(ty, format!("duplicate member `{ty}` (already listed as `{other}`)")));
        }
        seen_types.insert(ty.to_string(), ty.clone());

        let field_name = ty.to_string().to_snake_case();
        let field = format_ident!("{}", field_name, span = ty.span());

        if let Some(other) = seen_fields.get(&field_name) {
            return Err(Error::new_spanned(
                ty,
                format!("`{ty}` maps to field `{field_name}`, already used by `{other}`"),
            ));
        }
        seen_fields.insert(field_name, ty.clone());

        resolved.push(ConjunctionMember { ty: ty.clone(), field });
    }

    Ok(resolved)
}

pub fn emit_conjunction(
    attrs: &[Attribute],
    name: &Ident,
    members: &[Ident],
    kind: ProposeKind,
) -> Result<proc_macro2::TokenStream> {
    if members.is_empty() {
        return Err(Error::new_spanned(name, "conjunction requires at least one member"));
    }

    let resolved = resolve_conjunction_members(members)?;
    let member_tys: Vec<_> = resolved.iter().map(|m| &m.ty).collect();
    let member_fields: Vec<_> = resolved.iter().map(|m| &m.field).collect();

    Ok(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name {
            #( pub #member_fields: #member_tys, )*
        }

        impl crate::framework::Prop for #name {}
        impl crate::framework::Conjunction for #name {}

        impl #name {
            pub fn new(#( #member_fields: #member_tys ),*) -> Self {
                Self {
                    #( #member_fields, )*
                }
            }
        }

        impl crate::framework::Sorry for #name {
            fn sorry() -> Self {
                Self { #( #member_fields: <#member_tys as crate::framework::Sorry>::sorry(), )* }
            }
        }

        #(
            impl crate::framework::HasProp<#member_tys> for #name {
                fn prop(&self) -> #member_tys {
                    self.#member_fields
                }
            }
        )*
    })
}
