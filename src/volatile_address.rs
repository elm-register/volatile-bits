pub trait VolatileAddress<Addr> {
    fn address(&self) -> Addr;
}



macro_rules! impl_volatile_address {
    ($addr: ty) => {
        impl VolatileAddress<$addr>  for $addr{
            fn address(&self) -> $addr {
                *self
            }
        }
    };

    ($($addr: ty), *) => {
        $(impl_volatile_address!($addr);) *
    };
}


impl_volatile_address!(u16, u32, u64, usize);




#[cfg(test)]
mod tests {
    use crate::volatile_address::VolatileAddress;

    #[test]
    fn it_address() {
        assert_eq!(0xFFu16.address(), 0xFF);
        assert_eq!(0xFFu32.address(), 0xFF);
        assert_eq!(0xFFu64.address(), 0xFF);
        assert_eq!(0xFFusize.address(), 0xFF);
    }
}