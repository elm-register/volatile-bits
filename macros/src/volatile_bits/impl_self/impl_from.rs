use syn::__private::TokenStream2;

use crate::volatile_bits::config::VolatileBitsConfig;

pub fn expand_impl_from(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let ident = config.struct_name_ref();
    let address_ty = config.address_ty_ref();

    Ok(quote::quote! {
        impl From<#address_ty> for #ident{
            fn from(addr: #address_ty) -> Self{
                Self(addr)
            }
        }
    })
}