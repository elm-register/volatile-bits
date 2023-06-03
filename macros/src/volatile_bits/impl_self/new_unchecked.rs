use syn::__private::TokenStream2;
use crate::volatile_bits::config::VolatileBitsConfig;

pub fn expand_new_unchecked(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let address_ty = config.address_ty_ref();

    Ok(quote::quote! {
        pub fn new_unchecked(addr: #address_ty) -> Self{
            Self(addr)
        }
    })
}