use auto_delegate::{delegate, Delegate};

pub use builder::Builder;
pub use readonly::VolatileReadonly;
pub use write_only::VolatileWriteOnly;

mod write_only;
mod readonly;
mod builder;
pub(crate) mod config;


#[delegate]
pub trait VolatileBitsReadable<VolatileOut> {
    ///
    fn read_volatile(&self) -> VolatileOut;
}


#[delegate]
pub trait VolatileBitsWritable<VolatileIn> {
    fn write_volatile(&self, new_val: VolatileIn) -> core::result::Result<(), WriteErr>;
}

#[derive(Debug, Copy, Clone)]
pub enum WriteErr {
    ShlOverflow
}


#[derive(Delegate)]
pub struct Volatile<Addr, V> {
    #[to(VolatileBitsReadable)]
    read: VolatileReadonly<Addr, V>,

    #[to(VolatileBitsWritable)]
    write: VolatileWriteOnly<Addr, V>,
}


impl<Addr, V> Volatile<Addr, V> {
    pub(crate) const fn new(
        read: VolatileReadonly<Addr, V>,
        write: VolatileWriteOnly<Addr, V>,
    ) -> Volatile<Addr, V> {
        Self{
            read,
            write
        }
    }
}



