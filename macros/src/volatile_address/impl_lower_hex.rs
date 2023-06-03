use proc_macro2::{Ident, Span};
use quote::quote;
use syn::__private::TokenStream2;

use crate::volatile_address::config::VolatileAddressConfig;

pub fn expand_hex_impls(config: &VolatileAddressConfig) -> TokenStream2 {
    let ident = config.struct_name_ref();

    let impl_lower_hex = expand_impl_lower_hex(ident);
    let impl_upper_hex = expand_impl_upper_hex(ident);

    quote! {
        #impl_lower_hex
        #impl_upper_hex
    }
}


fn expand_impl_lower_hex(ident: &Ident) -> TokenStream2 {
    let impl_fmt = expand_impl_fmt(
        Ident::new("LowerHex", Span::call_site())
    );

    quote::quote! {
        impl core::fmt::LowerHex for #ident {
            #impl_fmt
        }
    }
}


fn expand_impl_upper_hex(ident: &Ident) -> TokenStream2 {
    let impl_fmt = expand_impl_fmt(
        Ident::new("UpperHex", Span::call_site())
    );

    quote::quote! {
        impl core::fmt::UpperHex for #ident {
            #impl_fmt
        }
    }
}

fn expand_impl_fmt(
    hex_ident: Ident
) -> TokenStream2 {
    quote::quote! {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            use volatile_bits::VolatileAddress;

            let val = self.address();
            core::fmt::#hex_ident::fmt(&val, f)
        }
    }
}

