use syn::__private::TokenStream2;

use crate::volatile_bits::config::VolatileBitsConfig;
use crate::volatile_bits::impl_self::address::expand_address;
use crate::volatile_bits::impl_self::impl_from::expand_impl_from;

mod impl_from;
mod address;


pub fn expand_impl_struct(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let struct_name = config.struct_name_ref();
    let impl_from = expand_impl_from(config)?;
    let address = expand_address(config)?;

    Ok(quote::quote! {
        impl #struct_name{
            #address
        }
        
        #impl_from
    })
}