use proc_macro2::{Ident, TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{parenthesized, LitStr, Token};

pub struct IdentList {
    pub parts: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _paren_token = parenthesized!(content in input);
        Ok(Self {
            parts: Punctuated::parse_terminated(&content)?,
        })
    }
}

/// Parses `("some text")`
pub struct AttrInnerSingleString {
    _paren_token: Paren,
    pub content: LitStr,
}

impl Parse for AttrInnerSingleString {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _paren_token = parenthesized!(content in input);
        Ok(Self {
            _paren_token,
            content: content.parse()?,
        })
    }
}

/// Parses `(left, right)`
pub struct Pair<F, S> {
    pub paren_token: Paren,
    pub first: F,
    pub sep: Token![,],
    pub second: S,
}

impl<F, S> Parse for Pair<F, S>
where
    F: Parse,
    S: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        Ok(Self {
            paren_token,
            first: content.parse()?,
            sep: content.parse()?,
            second: content.parse()?,
        })
    }
}

/// parses `(...)`
pub struct ParenthesizedTokens {
    pub paren: Paren,
    pub tokens: TokenStream,
}

impl Parse for ParenthesizedTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren = parenthesized!(content in input);
        Ok(Self {
            paren,
            tokens: content.parse()?,
        })
    }
}

/// parses a comma-delimeted list of `T`s with optional trailing comma
pub struct SimpleCommaDelimeted<T>(pub Vec<T>);

impl<T: Parse> Parse for SimpleCommaDelimeted<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punct = Punctuated::<T, Token![,]>::parse_terminated(input)?;
        Ok(Self(punct.into_iter().collect()))
    }
}
