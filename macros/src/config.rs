use proc_macro2::{Ident, Span};
use quote::__private::ext::RepToTokensExt;
use syn::{Fields, ItemStruct, Type};
use syn::__private::TokenStream2;

use crate::config::attribute::VolatileBitsAttribute;

mod attribute;

pub struct VolatileBitsConfig {
    struct_name: Ident,
    address_ty: Type,
    volatile_attr: VolatileBitsAttribute,
}


impl VolatileBitsConfig {
    pub fn new(item: &ItemStruct, attr_tokens: TokenStream2) -> syn::Result<Self> {
        Ok(Self {
            struct_name: item.ident.clone(),
            address_ty: address_type(&item.fields)?,
            volatile_attr: VolatileBitsAttribute::new(attr_tokens).unwrap_or_default(),
        })
    }


    pub fn struct_name_ref(&self) -> &Ident {
        &self.struct_name
    }


    pub fn address_ty_ref(&self) -> &Type {
        &self.address_ty
    }


    pub fn volatile_attr_ref(&self) -> &VolatileBitsAttribute {
        &self.volatile_attr
    }
}


fn address_type(fields: &Fields) -> syn::Result<Type> {
    if let Fields::Unnamed(field) = fields.next().ok_or(syn::Error::new(Span::call_site(), ""))? {
        return Ok(field.unnamed.first().unwrap().ty.clone());
    }

    Err(syn::Error::new(Span::call_site(), ""))
}
