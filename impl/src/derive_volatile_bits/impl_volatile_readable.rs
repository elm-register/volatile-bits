use syn::__private::TokenStream2;

use crate::config::VolatileBitsConfig;

pub fn expand_impl_volatile_readable(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let struct_name = config.struct_name_ref();
    let attr = config.volatile_attr_ref();
    let volatile_ty = attr.volatile_ty_ref();
    let read_volatile = expand_read_volatile(config);

    Ok(quote::quote! {
        impl volatile_bits::VolatileBitsReadable<#volatile_ty> for #struct_name{
            #read_volatile
        }
    })
}


fn expand_read_volatile(config: &VolatileBitsConfig) -> TokenStream2 {
    let attr = config.volatile_attr_ref();
    let offset = attr.offset_ref();
    let volatile_ty = attr.volatile_ty_ref();
    let bits = attr.bits_ref();
    let add = attr.add_ref();

    quote::quote! {
        fn read_volatile(&self) -> #volatile_ty{
            let mask = #volatile_ty::MAX >> (#volatile_ty::BITS - #bits);
            let v = unsafe{core::ptr::read_volatile((self.0 + #add) as *const #volatile_ty)} >> #offset;
            v & mask
        }
    }
}