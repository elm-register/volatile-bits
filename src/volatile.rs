pub use builder::Builder;
pub use readonly::VolatileReadonly;
pub use write_only::VolatileWriteOnly;

mod write_only;
mod readonly;
mod builder;
pub(crate) mod config;

pub trait VolatileBitsReadable<VolatileOut> {
    ///
    fn read_volatile(&self) -> VolatileOut;
}


pub trait VolatileBitsWritable<VolatileIn> {
    fn write_volatile(&self, new_val: VolatileIn) -> anyhow::Result<()>;
}

