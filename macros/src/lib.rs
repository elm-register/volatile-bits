extern crate alloc;
extern crate core;

use proc_macro::TokenStream;

use syn::__private::TokenStream2;

use crate::derive_volatile_bits::force_expand_volatile_bits;

mod config;
mod derive_volatile_bits;

#[proc_macro_attribute]
pub fn volatile_bits(attributes: TokenStream, input: TokenStream) -> TokenStream {
    union(proc_macro2::TokenStream::from(input.clone()), force_expand_volatile_bits(input, attributes))
}


fn union(lhs: TokenStream2, rhs: TokenStream2) -> TokenStream {
    let union = quote::quote! {
        #lhs
        #rhs
    };
    union.into()
}
