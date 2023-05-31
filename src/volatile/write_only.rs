use core::marker::PhantomData;

use crate::volatile::config::Config;

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


fn mask(max_val: usize, max_bits: usize, bits: usize, offset: usize) -> anyhow::Result<usize> {
    let bits = max_bits - bits;
    let mask = (max_val >> bits)
        .checked_shl(offset as u32)
        .ok_or(anyhow::anyhow!("Failed write_volatile overflow"))?;

    Ok(!mask)
}



macro_rules! impl_write_only_from_addr {
    ($addr: ty, $volatile: ty) => {
        impl crate::VolatileBitsWritable<$volatile> for VolatileWriteOnly<$addr, $volatile>
            {
                fn write_volatile(&self, new_val: $volatile) -> anyhow::Result<()> {
                    let old_val = unsafe{core::ptr::read_volatile(self.config.addr() as *const $volatile)};
                    let mask = mask(<$volatile>::MAX as usize, <$volatile>::BITS as usize, self.config.bits(), self.config.offset())? as $volatile;
                    let old_val_mask = old_val & mask;

                    let write_val = new_val.checked_shl(self.config.offset() as u32).ok_or(anyhow::anyhow!("Shl over flow!"))?;

                    unsafe{core::ptr::write_volatile(self.config.addr() as *mut $volatile, write_val | old_val_mask);}
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
    use crate::{VolatileBitsReadable, VolatileBitsWritable};
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
}