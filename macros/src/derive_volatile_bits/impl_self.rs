use syn::__private::TokenStream2;

use crate::config::VolatileBitsConfig;
use crate::derive_volatile_bits::impl_self::address::expand_address;
use crate::derive_volatile_bits::impl_self::new_unchecked::expand_new_unchecked;

mod new_unchecked;
mod address;


pub fn expand_impl_struct(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let struct_name = config.struct_name_ref();
    let new_unchecked = expand_new_unchecked(config)?;
    let address = expand_address(config)?;

    Ok(quote::quote! {
        impl #struct_name{
            #new_unchecked
            #address
        }
    })
}