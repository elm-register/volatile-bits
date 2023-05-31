pub mod write_only;
pub mod readonly;
pub mod builder;
pub(crate) mod config;


pub trait VolatileBitsReadable<VolatileOut>{
    ///
    fn read_volatile(&self) -> VolatileOut;
}


pub trait VolatileBitsWritable<VolatileIn>{
    fn write_volatile(&self, new_val: VolatileIn) -> anyhow::Result<()>;
}

