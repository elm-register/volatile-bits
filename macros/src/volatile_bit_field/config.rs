use alloc::vec::Vec;
use core::fmt::Debug;

use proc_macro2::{Ident, Span};
use syn::__private::TokenStream2;
use syn::ItemStruct;

use crate::volatile_bit_field::config::addr_ty::parse_addr_ty;
use crate::volatile_bit_field::config::name_field_config::{NameFieldConfig, parse_fields};

mod addr_ty;
mod name_field_config;

#[derive(Debug, Clone)]
pub struct VolatileBitFieldConfig {
    item: ItemStruct,
    addr_ty: Ident,
    fields: Vec<NameFieldConfig>,
}


impl VolatileBitFieldConfig {
    pub fn new(
        attrs: TokenStream2,
        item: ItemStruct,
    ) -> Self {
        let addr_ty = parse_addr_ty(attrs)
            .unwrap_or(Ident::new("u64", Span::call_site()));

        let fields = parse_fields(item.fields.clone());

        Self {
            item,
            addr_ty,
            fields,
        }
    }


    pub const fn ident_ref(&self) -> &Ident {
        &self.item.ident
    }


    pub const fn addr_ty_ref(&self) -> &Ident {
        &self.addr_ty
    }


    pub fn expand_getters(&self) -> TokenStream2{
        let getters = self
            .fields
            .iter()
            .map(NameFieldConfig::expand_getter);

        quote::quote!(#(#getters)*)
    }


    pub fn expand_impl_from_trait(&self) -> TokenStream2 {
        let fs = self
            .fields
            .iter()
            .map(|field| {
                let name = field.ident_ref();
                let ty = field.ty_ident_ref();

                quote::quote! {
                    #name : #ty::from(addr),
                }
            });

        let ident = self.ident_ref();
        let addr_ty = self.addr_ty_ref();
        quote::quote! {
            impl From<#addr_ty> for #ident {
                fn from(addr: #addr_ty) -> Self {
                    Self{
                        #(#fs)*
                    }
                }
            }
        }
    }
}







