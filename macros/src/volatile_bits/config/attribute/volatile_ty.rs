use alloc::string::ToString;

use proc_macro2::{Ident, TokenTree};
use syn::__private::TokenStream2;

pub fn parse_volatile_ty(volatile_attr_tokens: TokenStream2) -> Option<Ident> {
    let volatile_ty = volatile_attr_tokens
        .into_iter()
        .skip_while(is_not_type_attr)
        .nth(2)?;

    parse_type_value(volatile_ty)
}


fn parse_type_value(ty: TokenTree) -> Option<Ident> {
    if let TokenTree::Ident(ty) = ty {
        Some(ty)
    } else {
        None
    }
}

fn is_not_type_attr(tree: &TokenTree) -> bool {
    if let TokenTree::Ident(ident) = tree {
        ident.to_string().as_str() != "type"
    } else {
        true
    }
}
