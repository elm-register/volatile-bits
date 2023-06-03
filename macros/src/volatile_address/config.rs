use proc_macro2::Ident;
use syn::{ItemStruct, Type};

use crate::volatile_address::parse_new_type_field_ty;

#[derive(Debug, Clone)]
pub struct VolatileAddressConfig {
    struct_name: Ident,
    address_ty: Type,
}


impl VolatileAddressConfig {
    pub fn new(
        item: ItemStruct
    ) -> syn::Result<Self> {
        let address_ty = parse_new_type_field_ty(&item.fields)?;


        Ok(Self {
            struct_name: item.ident,
            address_ty,
        })
    }


    pub fn struct_name_ref(&self) -> &Ident {
        &self.struct_name
    }


    pub fn address_ty_ref(&self) -> &Type {
        &self.address_ty
    }
}