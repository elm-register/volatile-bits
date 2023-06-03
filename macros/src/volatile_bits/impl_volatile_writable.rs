use syn::__private::TokenStream2;
use crate::volatile_bits::config::VolatileBitsConfig;

pub fn expand_impl_volatile_writable(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let struct_name = config.struct_name_ref();
    let write_volatile = expand_write_volatile(config);

    let volatile_ty = config.volatile_attr_ref().volatile_ty_ref();
    Ok(quote::quote! {
        impl volatile_bits::VolatileBitsWritable<#volatile_ty> for #struct_name{
            #write_volatile
        }
    })
}


fn expand_write_volatile(config: &VolatileBitsConfig) -> TokenStream2 {
    let attr = config.volatile_attr_ref();

    let offset = attr.offset_ref();
    let bits = attr.bits_ref();
    let volatile_ty = attr.volatile_ty_ref();
    let add = attr.add_ref();

    quote::quote! {
        fn write_volatile(&self, new_val: #volatile_ty) -> anyhow::Result<()>{
            volatile_bits::volatile::Builder::new(self.0)
                .add_addr(#add)
                .bits(#bits as usize)
                .offset(#offset)
                .build_write_only::<#volatile_ty>()
                .write_volatile(new_val)
        }
    }
}