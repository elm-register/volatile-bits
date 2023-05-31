#![cfg_attr(not(test), no_std)]

pub use volatile::{VolatileBitsReadable, VolatileBitsWritable};
pub use volatile_bits_impl::volatile_bits;

pub mod numeric;
pub mod volatile;

