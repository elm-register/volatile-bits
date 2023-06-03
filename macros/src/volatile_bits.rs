use proc_macro::TokenStream;

use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::volatile_bits::config::VolatileBitsConfig;
use crate::volatile_bits::impl_self::expand_impl_struct;
use crate::volatile_bits::impl_volatile_readable::expand_impl_volatile_readable;
use crate::volatile_bits::impl_volatile_writable::expand_impl_volatile_writable;

mod impl_self;
mod impl_volatile_readable;
mod impl_volatile_writable;
mod config;

pub fn force_expand_volatile_bits(item: TokenStream, attr_tokens: TokenStream) -> TokenStream2 {
    expand_volatile_bits(item, TokenStream2::from(attr_tokens))
        .unwrap_or_else(|e| e.to_compile_error())
}


fn expand_volatile_bits(item: TokenStream, attr_tokens: TokenStream2) -> syn::Result<TokenStream2> {
    let item = syn::parse::<ItemStruct>(item)?;
    let config = VolatileBitsConfig::new(&item, attr_tokens)?;

    let impl_self = expand_impl_struct(&config)?;
    let impl_volatile_readable = expand_impl_volatile_readable(&config)?;
    let impl_volatile_writable = expand_impl_volatile_writable(&config)?;

    Ok(quote::quote! {
        #impl_self
        #impl_volatile_readable
        #impl_volatile_writable
    })
}




