use alloc::string::ToString;
use proc_macro2::{Literal, TokenTree};
use syn::__private::TokenStream2;

pub fn parse_add(volatile_attr_tokens: TokenStream2) -> Option<Literal> {
    let add = volatile_attr_tokens
        .into_iter()
        .skip_while(is_not_add_attr)
        .nth(2)?;

    parse_add_value(add)
}


fn parse_add_value(ty: TokenTree) -> Option<Literal> {
    if let TokenTree::Literal(add_address_bytes) = ty {
        Some(add_address_bytes)
    } else {
        None
    }
}

fn is_not_add_attr(tree: &TokenTree) -> bool {
    if let TokenTree::Ident(ident) = tree {
        let ident: &proc_macro2::Ident = ident;
        ident.to_string().as_str() != "add"
    } else {
        true
    }
}
