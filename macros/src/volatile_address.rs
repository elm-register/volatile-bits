use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::__private::ext::RepToTokensExt;
use syn::{Fields, FieldsUnnamed, ItemStruct};
use syn::__private::TokenStream2;

use crate::volatile_address::config::VolatileAddressConfig;
use crate::volatile_address::impl_lower_hex::expand_hex_impls;

mod config;
mod impl_lower_hex;

pub fn expand_volatile_address(item: TokenStream) -> TokenStream2 {
    try_expand_volatile_address(item)
        .unwrap_or_else(|e| e.to_compile_error())
}


fn try_expand_volatile_address(item: TokenStream) -> syn::Result<TokenStream2> {
    let item = syn::parse::<ItemStruct>(item)?;
    let config = VolatileAddressConfig::new(item)?;

    let impl_volatile_address = expand_impl_volatile_address(&config);

    let impl_from_address_ty = expand_impl_from_address_ty(&config);

    let impls_hex = expand_hex_impls(&config);

    Ok(quote::quote! {
        #impl_volatile_address
        #impl_from_address_ty
        #impls_hex
    })
}


fn expand_impl_from_address_ty(config: &VolatileAddressConfig) -> TokenStream2 {
    let ident = config.struct_name_ref();

    let address_ty = config.address_ty_ref();

    quote::quote! {
        impl From<#address_ty> for #ident{
            fn from(address: #address_ty) -> #ident{
                Self(address)
            }
        }
    }
}


fn expand_impl_volatile_address(
    config: &VolatileAddressConfig
) -> TokenStream2 {
    let struct_ident = config.struct_name_ref();
    let address_ty = config.address_ty_ref();

    quote::quote! {
        impl volatile_bits::VolatileAddress<#address_ty> for #struct_ident{
            fn address(&self) -> #address_ty{
                self.0
            }
        }
    }
}


fn parse_new_type_field_ty(fields: &Fields) -> syn::Result<syn::Type> {
    if fields.len() != 1 {
        return Err(syn::Error::new(Span::call_site(), "TODO"));
    }

    let field = fields
        .next()
        .ok_or(error_not_found_field())?;

    match field {
        Fields::Unnamed(unnamed) => unnamed_field_ty(unnamed),
        _ => Err(error_not_unnamed_field())
    }
}


fn unnamed_field_ty(unnamed: &FieldsUnnamed) -> syn::Result<syn::Type> {
    let field = unnamed
        .unnamed
        .iter()
        .next()
        .ok_or(error_not_found_field())?;

    Ok(field.ty.clone())
}


fn error_not_found_field() -> syn::Error {
    syn::Error::new(Span::call_site(), "Not found field")
}


fn error_not_unnamed_field() -> syn::Error {
    syn::Error::new(Span::call_site(), "Not found Unnamed Field")
}