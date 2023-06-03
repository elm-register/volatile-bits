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
    let mode = config
        .volatile_attr_ref()
        .access_mode();

    let impl_volatile_readable = expand_read_volatile_if_need(mode.is_write_only(), &config)?;
    let impl_volatile_writable = expand_write_volatile_if_need(mode.is_readonly(), &config)?;

    Ok(quote::quote! {
        #impl_self
        #impl_volatile_readable
        #impl_volatile_writable
    })
}


fn expand_read_volatile_if_need(write_only: bool, config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    if write_only {
        Ok(quote::quote!())
    } else {
        expand_impl_volatile_readable(config)
    }
}



fn expand_write_volatile_if_need(readonly: bool, config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    if readonly {
        Ok(quote::quote!())
    } else {
        expand_impl_volatile_writable(config)
    }
}
