use proc_macro2::{Span, TokenTree};
use syn::__private::TokenStream2;

use crate::volatile_bits::config::attribute::ne_attr_name;

#[derive(Debug, Copy, Clone)]
pub enum AccessMode {
    Readonly,
    WriteOnly,
    ReadWrite,
}

impl AccessMode {
    pub fn new(attrs: TokenStream2) -> syn::Result<Self> {
        parse_mem_attr(attrs)
    }


    pub fn is_readonly(&self) -> bool {
        matches!(self, AccessMode::Readonly)
    }


    pub fn is_write_only(&self) -> bool {
        matches!(self, AccessMode::WriteOnly)
    }
}

pub fn parse_mem_attr(attrs: TokenStream2) -> syn::Result<AccessMode> {
    if let Some(mem) = attrs
        .into_iter()
        .skip_while(|tree| ne_attr_name(&tree, "mem"))
        .nth(2) {
        parse_mem_value(mem)
    } else {
        Ok(AccessMode::ReadWrite)
    }
}


fn parse_mem_value(mem: TokenTree) -> syn::Result<AccessMode> {
    let attr_error = || {
        syn::Error::new(Span::call_site(), "Only 'readonly' and 'write_only' can be specified for mem")
    };
    if let TokenTree::Ident(mode) = mem {
        match mode.to_string().as_str() {
            "readonly" => Ok(AccessMode::Readonly),
            "write_only" => Ok(AccessMode::WriteOnly),
            _ => Err(attr_error())
        }
    } else {
        Err(attr_error())
    }
}





