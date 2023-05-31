use core::marker::PhantomData;

use crate::VolatileBitsReadable;

pub struct VolatileBitsReadonly<Addr, Volatile> {
    addr: Addr,
    offset: usize,
    bits: usize,
    add_addr: usize,
    _maker: PhantomData<Volatile>,
}

impl<Addr, Volatile> VolatileBitsReadonly<Addr, Volatile> {
    pub(crate) const fn new(
        addr: Addr,
        offset: usize,
        bits: usize,
        add_addr: usize,
    ) -> VolatileBitsReadonly<Addr, Volatile> {
        Self {
            addr,
            offset,
            bits,
            add_addr,
            _maker: PhantomData,
        }
    }
}

macro_rules! impl_readable_from_addr {
    ($addr: ty, $volatile: ty) => {
        impl VolatileBitsReadable<$volatile> for VolatileBitsReadonly<$addr, $volatile>{
            fn read_volatile(&self) -> $volatile {
                let max_bits = <$volatile>::BITS as usize;
                let mask = <$volatile>::MAX >> (max_bits - self.bits);

                let v = unsafe{core::ptr::read_volatile((self.addr + self.add_addr as $addr) as *const $volatile)} >> self.offset;
                v & mask as $volatile
            }
        }
    };


    ($addr: ty, $(volatile: ty),*) => {
        $(impl_readable_from_addr!($addr, $volatile));*
    };
}

macro_rules! impl_readable {
    ($($addr: ty), *) => {
        $(
            impl_readable_from_addr!($addr, u8);
            impl_readable_from_addr!($addr, u16);
            impl_readable_from_addr!($addr, u64);
            impl_readable_from_addr!($addr, usize);
            impl_readable_from_addr!($addr, u128);
        )*
    };
}

impl_readable!(u8, u16, u32, u64, usize, u128);

#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::VolatileBitsReadable;

    #[test]
    fn it_read_volatile() {
        let buff: [u8; 1] = [0x31];

        let v = Builder::new(buff.as_ptr() as u64).build_readonly::<u8>();

        assert_eq!(v.read_volatile(), 0x31);
    }


    #[test]
    fn it_read_volatile_with_offset() {
        let buff: [u8; 1] = [0b1001];

        let v = Builder::new(buff.as_ptr() as u64)
            .offset(1)
            .build_readonly::<u8>();

        assert_eq!(v.read_volatile(), 0b100);
    }


    #[test]
    fn it_read_volatile_with_bits() {
        let buff: [u64; 1] = [0b11_0110];

        let v = Builder::new(buff.as_ptr() as u64)
            .offset(1)
            .bits(5)
            .build_readonly::<u64>();

        assert_eq!(v.read_volatile(), 0b11_011);
    }



        #[test]
    fn it_read_volatile_with_bits2() {
        let buff: [u64; 1] = [0b1111_0110];

        let v = Builder::new(buff.as_ptr() as u64)
            .bits(4)
            .build_readonly::<u64>();

        assert_eq!(v.read_volatile(), 0b0110);
    }


    #[test]
    fn it_read_volatile_with_add_addr() {
        let buff: [u8; 3] = [1, 2, 3];

        let v = Builder::new(buff.as_ptr() as u64)
            .add_addr(1)
            .build_readonly::<u8>();

        assert_eq!(v.read_volatile(), 2);
    }


    #[test]
    fn it_read_volatile_with_all_options() {
        let buff: [u8; 3] = [0, 0, 0b1011_0101];

        let v = Builder::new(buff.as_ptr() as u64)
            .add_addr(2)
            .offset(4)
            .bits(3)
            .build_readonly::<u8>();

        assert_eq!(v.read_volatile(), 0b011);
    }
}
