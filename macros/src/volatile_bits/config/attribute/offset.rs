use alloc::string::ToString;
use proc_macro2::{Literal, TokenTree};
use syn::__private::TokenStream2;

pub fn parse_offset_token(volatile_attr_tokens: TokenStream2) -> Option<Literal> {
    let offset = volatile_attr_tokens
        .into_iter()
        .skip_while(is_not_offset_attr)
        .nth(2)?;

    parse_offset_value(offset)
}


fn parse_offset_value(offset: TokenTree) -> Option<Literal> {
    if let TokenTree::Literal(lit) = offset {
        Some(lit)
    } else {
        None
    }
}

fn is_not_offset_attr(tree: &TokenTree) -> bool {
    if let TokenTree::Ident(ident) = tree {
        ident.to_string().as_str() != "offset"
    } else {
        true
    }
}

