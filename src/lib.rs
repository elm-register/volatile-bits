#![no_std]

pub mod readonly;
pub mod builder;
pub mod numeric;

pub use volatile_bits_impl::volatile_bits;

pub trait VolatileBitsReadable<VolatileOut>{
    ///
    fn read_volatile(&self) -> VolatileOut;
}


pub trait VolatileBitsWritable<VolatileIn>{
    fn write_volatile(&self, new_val: VolatileIn);
}

