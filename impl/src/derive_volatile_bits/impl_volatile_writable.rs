use syn::__private::TokenStream2;
use crate::config::VolatileBitsConfig;

pub fn expand_impl_volatile_writable(config: &VolatileBitsConfig) -> syn::Result<TokenStream2> {
    let struct_name = config.struct_name_ref();
    let write_volatile = expand_write_volatile();

    Ok(quote::quote! {
        impl volatile_bits::VolatileBitsWritable<u64> for #struct_name{
            #write_volatile
        }
    })
}


fn expand_write_volatile() -> TokenStream2 {
    quote::quote! {
        fn write_volatile(&self, new_val: u64){
            unsafe{core::ptr::write_volatile(self.0 as *mut u64, new_val)}
        }
    }
}