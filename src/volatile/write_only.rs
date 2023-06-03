use core::marker::PhantomData;

use crate::volatile::config::Config;
use crate::volatile::WriteErr;

pub struct VolatileWriteOnly<Addr, Volatile> {
    config: Config<Addr>,
    _marker: PhantomData<Volatile>,
}


impl<Addr, Volatile> VolatileWriteOnly<Addr, Volatile> {
    pub(crate) const fn new(
        config: Config<Addr>
    ) -> VolatileWriteOnly<Addr, Volatile> {
        Self {
            config,
            _marker: PhantomData,
        }
    }
}


fn mask(max_val: usize, max_bits: usize, bits: usize, offset: usize) -> core::result::Result<usize, WriteErr> {
    let bits = max_bits - bits;
    let mask = (max_val >> bits)
        .checked_shl(offset as u32)
        .ok_or(WriteErr::ShlOverflow)?;

    Ok(!mask)
}



macro_rules! impl_write_only_from_addr {
    ($addr: ty, $volatile: ty) => {
        impl crate::VolatileBitsWritable<$volatile> for VolatileWriteOnly<$addr, $volatile>
            {
                fn write_volatile(&self, new_val: $volatile) -> core::result::Result<(), crate::WriteErr> {

                    let mask = mask(<$volatile>::MAX as usize, <$volatile>::BITS as usize, self.config.bits(), self.config.offset())? as $volatile;
                    let addr = self.config.addr() + self.config.add_addr() as $addr;

                    let old_val = unsafe{core::ptr::read_volatile(addr as *const $volatile)};
                    let old_val_mask = old_val & mask;

                    let write_val = new_val.checked_shl(self.config.offset() as u32)
                        .ok_or(crate::WriteErr::ShlOverflow)?;

                    unsafe{core::ptr::write_volatile(addr as *mut $volatile, write_val | old_val_mask);}
                    Ok(())
                }
            }
    };


    ($addr: ty, $(volatile: ty),*) => {
        $(impl_write_only_from_addr!($addr, $volatile));*
    };
}

macro_rules! impl_write_only {
    ($($addr: ty), *) => {
        $(
            impl_write_only_from_addr!($addr, u8);
            impl_write_only_from_addr!($addr, u16);
            impl_write_only_from_addr!($addr, u32);
            impl_write_only_from_addr!($addr, u64);
            impl_write_only_from_addr!($addr, usize);
            impl_write_only_from_addr!($addr, u128);
        )*
    };
}

impl_write_only!(u8, u16, u32, u64, usize, u128);


#[cfg(test)]
mod tests {
    use crate::{VolatileAddress, VolatileBitsReadable, VolatileBitsWritable};
    use crate::numeric::Numeric;
    use crate::volatile::builder::Builder;
    use crate::volatile::write_only::mask;

    #[test]
    fn it_mask() {
        let mask = mask(u8::MAX as usize, u8::max_bits(), u8::max_bits(), 0).unwrap();
        assert_eq!(mask as u8, 0b0000_0000);
    }

    #[test]
    fn it_mask_offset1() {
        let mask = mask(u8::MAX as usize, u8::max_bits(), u8::max_bits(), 1).unwrap();
        assert_eq!(mask as u8, 0b0000_0001);
    }


    #[test]
    fn it_mask_bits4() {
        let mask = mask(u16::MAX as usize, u16::max_bits(), 4, 0).unwrap();
        assert_eq!(mask as u16, 0b1111_1111_1111_0000);
    }


    #[test]
    fn it_mask_with_offset_and_bits() {
        let mask = mask(u8::MAX as usize, u8::max_bits(), 3, 1).unwrap();
        assert_eq!(mask as u8, 0b1111_0001);
    }


    #[test]
    fn it_write_volatile() {
        let buff: [u32; 1] = [0; 1];

        let w = Builder::new(buff.as_ptr() as u64)
            .build_write_only::<u32>();
        w.write_volatile(0b100).unwrap();

        let r = Builder::new(buff.as_ptr() as u64)
            .build_readonly::<u32>();
        assert_eq!(r.read_volatile(), 0b100);
    }


    #[test]
    fn it_write_volatile_with_offset() {
        let buff: [u32; 1] = [0b0000_1001; 1];

        let builder = Builder::new(buff.as_ptr() as u64)
            .offset(4);

        let w = builder
            .clone()
            .build_write_only::<u32>();
        w.write_volatile(0b100).unwrap();

        let r = builder
            .build_readonly::<u32>();
        assert_eq!(r.read_volatile(), 0b100);

        assert_eq!(buff[0], 0b0100_1001);
    }


    #[test]
    fn it_write_volatile_with_bits() {
        let buff: [u32; 1] = [0b1111_1111; 1];

        let w = Builder::new(buff.as_ptr() as u64)
            .bits(3)
            .build_write_only::<u32>();
        w.write_volatile(0b000).unwrap();

        assert_eq!(buff[0], 0b1111_1000);
    }


    #[test]
    fn it_write_volatile_with_bits_and_offset() {
        let buff: [u16; 1] = [0b1111_1111; 1];

        let w = Builder::new(buff.as_ptr() as u64)
            .offset(2)
            .bits(3)
            .build_write_only::<u16>();
        w.write_volatile(0b000).unwrap();

        assert_eq!(buff[0], 0b1110_0011);
    }


    #[test]
    fn it_write_volatile_with_add_addr() {
        let buff: [u8; 5] = [1, 2, 3, 4, 5];

        let w = Builder::new(buff.as_ptr() as u64)
            .add_addr(2)
            .build_write_only::<u8>();

        w.write_volatile(100).unwrap();

        assert_eq!(buff[0], 1);
        assert_eq!(buff[1], 2);
        assert_eq!(buff[2], 100);
        assert_eq!(buff[3], 4);
        assert_eq!(buff[4], 5);
    }


    #[test]
    fn it_write_volatile_with_all_options() {
        let buff: [u8; 5] = [1, 2, 0b0000_0000, 4, 5];

        let w = Builder::new(buff.as_ptr() as u64)
            .add_addr(2)
            .bits(3)
            .offset(5)
            .build_write_only::<u8>();

        w.write_volatile(0b111).unwrap();

        assert_eq!(buff[0], 1);
        assert_eq!(buff[1], 2);
        assert_eq!(buff[2], 0b1110_0000);
        assert_eq!(buff[3], 4);
        assert_eq!(buff[4], 5);
    }


    #[test]
    fn it_write() {
        let buff: [u8; 3] = [1, 0b1111_1111, 3];
        let b = Builder::new(buff.as_ptr() as u64)
            .offset(2)
            .bits(3)
            .add_addr(0x01)
            .build_write_only::<u8>();

        b.write_volatile(0b000).unwrap();

        assert_eq!(buff[1], 0b1110_0011)
    }


    #[test]
    fn it_new_type_address() {
        struct Address(u64);
        impl VolatileAddress<u64> for Address {
            fn address(&self) -> u64 {
                self.0
            }
        }

        let buff: [u8; 3] = [1, 0b1111_1111, 3];

        let addr = Address(buff.as_ptr() as u64);

        let b = Builder::new(addr)
            .offset(2)
            .bits(3)
            .add_addr(0x01)
            .build_write_only::<u8>();

        b.write_volatile(0b000).unwrap();

        assert_eq!(buff[1], 0b1110_0011)
    }
}