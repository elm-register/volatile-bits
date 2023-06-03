use proc_macro2::{Ident, TokenTree};
use syn::__private::TokenStream2;

pub fn parse_addr_ty(attrs: TokenStream2) -> Option<Ident> {
    let addr_ty_tree = attrs
        .into_iter()
        .skip_while(not_addr_ty)
        .nth(2)?;


    addr_ty_value(addr_ty_tree)
}


fn addr_ty_value(addr_ty_tree: TokenTree) -> Option<Ident> {
    if let TokenTree::Ident(addr_ty) = addr_ty_tree {
        Some(addr_ty)
    } else {
        None
    }
}

fn not_addr_ty(tree: &TokenTree) -> bool {
    if let TokenTree::Ident(tree) = tree {
        *tree != "addr_ty"
    } else {
        true
    }
}