use proc_macro2::{Ident, Span};

/// Creates an ident at the call site
pub fn call_site_ident<S: AsRef<str>>(s: S) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}
