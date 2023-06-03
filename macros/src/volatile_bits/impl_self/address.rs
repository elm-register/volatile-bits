use syn::__private::TokenStream2;
use crate::volatile_bits::config::VolatileBitsConfig;

pub fn expand_address(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let address_ty = config.address_ty_ref();

    Ok(quote::quote! {
        pub fn address(&self) -> #address_ty{
            self.0
        }
    })
}