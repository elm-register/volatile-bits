use proc_macro2::{Ident, Literal, Span};
use syn::__private::TokenStream2;

use crate::volatile_bits::config::attribute::add::parse_add;
use crate::volatile_bits::config::attribute::bits::parse_bits;
use crate::volatile_bits::config::attribute::offset::parse_offset_token;
use crate::volatile_bits::config::attribute::volatile_ty::parse_volatile_ty;

mod offset;
mod volatile_ty;
mod bits;
mod add;

#[derive(Clone)]
pub struct VolatileBitsAttribute {
    offset: Literal,
    volatile_ty: Ident,
    bits: TokenStream2,
    add_address_bytes: Literal,
}


impl Default for VolatileBitsAttribute {
    fn default() -> Self {
        Self {
            offset: Literal::usize_unsuffixed(0),
            volatile_ty: Ident::new("u64", Span::call_site()),
            bits: quote::quote!(64),
            add_address_bytes: Literal::usize_unsuffixed(0),
        }
    }
}


impl VolatileBitsAttribute {
    pub fn new(tokens: TokenStream2) -> Option<Self> {
        let offset = parse_offset_token(tokens.clone())
            .unwrap_or(Literal::usize_unsuffixed(0));

        let volatile_ty = parse_volatile_ty(tokens.clone())
            .unwrap_or(Ident::new("u64", Span::call_site()));

        let bits = parse_bits(tokens.clone())
            .unwrap_or(volatile_ty_to_bits(&volatile_ty));

        let add_address_bytes = parse_add(tokens)
            .unwrap_or(Literal::usize_unsuffixed(0));

        Some(Self {
            offset,
            volatile_ty,
            bits,
            add_address_bytes,
        })
    }


    pub fn offset_ref(&self) -> &Literal {
        &self.offset
    }


    pub fn volatile_ty_ref(&self) -> &Ident {
        &self.volatile_ty
    }


    pub fn bits_ref(&self) -> &TokenStream2 {
        &self.bits
    }


    pub fn add_ref(&self) -> &Literal {
        &self.add_address_bytes
    }
}


fn volatile_ty_to_bits(volatile_ty: &Ident) -> TokenStream2 {
    quote::quote! {
        #volatile_ty::BITS
    }
}