use syn::Ident;

pub const ABSURD: &str = "Absurd";

pub fn is_keyword(ident: &Ident) -> bool {
    ident == ABSURD
}
