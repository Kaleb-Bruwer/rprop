use syn::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropExpr {
    Atom(Ident),
    And(Vec<Box<PropExpr>>),
    Or(Vec<Box<PropExpr>>),
}

#[derive(Debug, Clone)]
pub enum NamedExpr {
    Atom,
    And(Vec<Box<NamedPropExpr>>),
    Or(Vec<Box<NamedPropExpr>>),
}

#[derive(Clone)]
pub struct ProposeInput {
    pub attrs: Vec<syn::Attribute>,
    pub name: Ident,
    pub expr: Option<PropExpr>,
}

#[derive(Debug, Clone)]
pub struct NamedPropExpr {
    pub name: Ident,
    pub expr: NamedExpr,
}

impl NamedPropExpr {
    pub fn collect_postorder<'a>(&'a self, out: &mut Vec<&'a NamedPropExpr>) {
        match &self.expr {
            NamedExpr::Atom => {}
            NamedExpr::And(children) | NamedExpr::Or(children) => {
                for child in children {
                    child.collect_postorder(out);
                }
            }
        }
        out.push(self);
    }
}
