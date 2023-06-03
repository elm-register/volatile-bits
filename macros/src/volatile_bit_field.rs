use proc_macro::TokenStream;

use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::volatile_bit_field::config::VolatileBitFieldConfig;

mod config;

pub fn expand_volatile_bit_field(attr: TokenStream, item: TokenStream) -> TokenStream2 {
    try_expand_volatile_bit_field(TokenStream2::from(attr), item)
        .unwrap_or_else(|e| e.to_compile_error())
}


fn try_expand_volatile_bit_field(
    attrs: TokenStream2,
    item: TokenStream,
) -> syn::Result<TokenStream2> {
    let item = syn::parse::<ItemStruct>(item)?;
    let config = VolatileBitFieldConfig::new(attrs, item);

    let impl_from = config.expand_impl_from_trait();
    let impl_getters = config.expand_getters();
    let ident = config.ident_ref();

    Ok(quote::quote! {
        impl #ident{
            #impl_getters
        }

        #impl_from
    })
}








