use proc_macro2::Ident;
use syn::{Field, Fields, Type};

pub fn parse_fields(fields: Fields) -> Vec<NameFieldConfig> {
    fields
        .iter()
        .filter_map(|field| NameFieldConfig::new(field.clone()))
        .collect()
}


#[derive(Debug, Clone)]
pub struct NameFieldConfig {
    ident: Ident,
    ty_ident: Type,
}


impl NameFieldConfig {
    pub fn new(field: Field) -> Option<Self> {
        Some(Self {
            ident: field.ident?.clone(),
            ty_ident: field.ty,
        })
    }


    pub const fn ident_ref(&self) -> &Ident {
        &self.ident
    }


    pub const fn ty_ident_ref(&self) -> &Type {
        &self.ty_ident
    }
}