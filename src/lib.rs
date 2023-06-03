#![no_std]

pub use volatile::{VolatileBitsReadable, VolatileBitsWritable, WriteErr};
pub use volatile_address::VolatileAddress;
pub use volatile_bits_macros::{volatile_address, volatile_bit_field, volatile_bits};

pub mod numeric;
pub mod volatile;
mod volatile_address;
