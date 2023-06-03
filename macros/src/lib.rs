#![no_std]
extern crate alloc;
extern crate core;


use proc_macro::TokenStream;

use syn::__private::TokenStream2;

use crate::volatile_address::expand_volatile_address;
use crate::volatile_bit_field::expand_volatile_bit_field;
use crate::volatile_bits::force_expand_volatile_bits;

mod volatile_bits;
mod volatile_address;
mod volatile_bit_field;


#[proc_macro_attribute]
pub fn volatile_address(_attributes: TokenStream, item: TokenStream) -> TokenStream {
    join(
        join_address_derives(item.clone()),
        expand_volatile_address(item),
    )
        .into()
}


#[proc_macro_attribute]
pub fn volatile_bits(attributes: TokenStream, input: TokenStream) -> TokenStream {
    join(
        proc_macro2::TokenStream::from(input.clone()),
        force_expand_volatile_bits(input, attributes),
    ).into()
}


#[proc_macro_attribute]
pub fn volatile_bit_field(attributes: TokenStream, item: TokenStream) -> TokenStream {
    join(
        proc_macro2::TokenStream::from(item.clone()),
        expand_volatile_bit_field(attributes, item),
    ).into()
}


fn join_address_derives(item: TokenStream) -> TokenStream2 {
    join(
        quote::quote! {
            #[derive(
                core::fmt::Debug,
                core::marker::Copy,
                core::clone::Clone,
                Ord,
                PartialOrd,
                Eq,
                PartialEq,
                core::hash::Hash
            )]
            #[repr(transparent)]
        },
        TokenStream2::from(item),
    )
}


#[derive(core::fmt::Debug)]
struct A;

fn join(lhs: TokenStream2, rhs: TokenStream2) -> TokenStream2 {
    let union = quote::quote! {
        #lhs
        #rhs
    };
    union
}
