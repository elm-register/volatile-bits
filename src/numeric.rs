
pub trait Numeric {
    fn max_bits() -> usize;
}

macro_rules! impl_numeric {
    ($numeric: ty) => {
        impl crate::numeric::Numeric for $numeric{
            fn max_bits() -> usize {
                <$numeric>::BITS as usize
            }
        }
    };


    ($($numeric: ty), *) => {
        $(impl_numeric!($numeric);)*
    }
}



impl_numeric!(u8, u16, u32, u64, usize, u128);


#[cfg(test)]
mod tests{
    use crate::numeric::Numeric;

    #[test]
    fn it_numerics(){
        macro_rules! assert_numeric {
            ($numeric: ident) => {
                  assert_eq!($numeric::max_bits(), $numeric::BITS as usize);
            };

            ($($numeric: ident), *) => {
                $(assert_numeric!($numeric);)*
            }
        }


        assert_numeric!(u8, u16, u32, u64, usize, u128);
    }
}