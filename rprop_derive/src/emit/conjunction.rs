use std::collections::HashMap;
use std::fmt;

use quote::quote;
use syn::{Attribute, Error, Ident, Result};

use crate::{
    ast::{Atom, NamedExpr, NatArg},
    generics::{emit_const_arg_list, emit_const_params, GenericContext},
    nat,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConjunctionMemberArg {
    Lit(nat::Nat),
    Param(Ident),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConjunctionMemberKey {
    pub name: Ident,
    pub args: Vec<ConjunctionMemberArg>,
}

impl ConjunctionMemberKey {
    pub fn from_atom(atom: &Atom) -> Self {
        Self {
            name: atom.name.clone(),
            args: atom
                .args
                .iter()
                .map(|arg| match arg {
                    NatArg::Lit(n) => ConjunctionMemberArg::Lit(*n),
                    NatArg::Param(ident) => ConjunctionMemberArg::Param(ident.clone()),
                })
                .collect(),
        }
    }

    pub fn from_named_expr(expr: &NamedExpr) -> Self {
        match expr {
            NamedExpr::Atom(atom) => Self::from_atom(atom),
            named => Self {
                name: named.name(),
                args: named.collect_param_binders().into_iter().map(ConjunctionMemberArg::Param).collect(),
            },
        }
    }
}

impl fmt::Display for ConjunctionMemberArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConjunctionMemberArg::Lit(n) => write!(f, "{n}"),
            ConjunctionMemberArg::Param(ident) => write!(f, "{ident}"),
        }
    }
}

impl fmt::Display for ConjunctionMemberKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.args.is_empty() {
            write!(f, "<")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{arg}")?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

pub struct ConjunctionMember {
    pub ty: proc_macro2::TokenStream,
    pub field: Ident,
}

pub fn resolve_conjunction_members(members: &[&NamedExpr]) -> Result<Vec<ConjunctionMember>> {
    let mut seen_types: HashMap<ConjunctionMemberKey, Ident> = HashMap::new();
    let mut seen_fields: HashMap<Ident, Ident> = HashMap::new();
    let mut resolved = Vec::new();

    for member in members {
        let key = ConjunctionMemberKey::from_named_expr(member);
        if let Some(other) = seen_types.get(&key) {
            return Err(Error::new_spanned(
                &member.name(),
                format!("duplicate member `{key}` (already listed as `{other}`)"),
            ));
        }
        seen_types.insert(key, member.name());

        let field = member.member_field_name();
        if let Some(other) = seen_fields.get(&field) {
            return Err(Error::new_spanned(
                &member.name(),
                format!("`{}` maps to field `{field}`, already used by `{other}`", member.name()),
            ));
        }
        seen_fields.insert(field.clone(), member.name());

        resolved.push(ConjunctionMember { ty: member.reference_tokens(), field });
    }

    Ok(resolved)
}

pub fn emit_conjunction(
    attrs: &[Attribute],
    name: &Ident,
    members: &[&NamedExpr],
    ctx: &GenericContext,
) -> Result<proc_macro2::TokenStream> {
    if members.is_empty() {
        return Err(Error::new_spanned(name, "conjunction requires at least one member"));
    }

    let resolved = resolve_conjunction_members(members)?;
    let const_params = emit_const_params(&ctx.binders);
    let const_args = emit_const_arg_list(&ctx.binders);
    let member_tys: Vec<_> = resolved.iter().map(|m| &m.ty).collect();
    let member_fields: Vec<_> = resolved.iter().map(|m| &m.field).collect();

    Ok(quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        pub struct #name #const_params {
            #( pub #member_fields: #member_tys, )*
        }

        impl #const_params ::rprop::Prop for #name #const_args {}
        impl #const_params ::rprop::Conjunction for #name #const_args {}

        impl #const_params #name #const_args {
            pub fn new(#( #member_fields: #member_tys ),*) -> Self {
                Self {
                    #( #member_fields, )*
                }
            }
        }

        #(
            impl #const_params ::rprop::HasProp<#member_tys> for #name #const_args {
                fn prop(&self) -> #member_tys {
                    self.#member_fields
                }
            }
        )*
    })
}
