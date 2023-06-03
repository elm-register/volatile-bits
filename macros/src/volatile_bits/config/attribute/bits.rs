use alloc::string::ToString;
use proc_macro2::TokenTree;
use syn::__private::TokenStream2;

pub fn parse_bits(volatile_attr_tokens: TokenStream2) -> Option<TokenStream2> {
    let bits = volatile_attr_tokens
        .into_iter()
        .skip_while(is_not_bits_attr)
        .nth(2)?;

    parse_bits_value(bits)
}


fn parse_bits_value(ty: TokenTree) -> Option<TokenStream2> {
    if let TokenTree::Literal(bits) = ty {
        Some(quote::quote!(#bits))
    } else {
        None
    }
}


fn is_not_bits_attr(tree: &TokenTree) -> bool {
    if let TokenTree::Ident(ident) = tree {
        ident.to_string().as_str() != "bits"
    } else {
        true
    }
}
