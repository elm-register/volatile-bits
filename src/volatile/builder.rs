use core::marker::PhantomData;

use crate::numeric::Numeric;
use crate::volatile::config::Config;
use crate::volatile::readonly::VolatileReadonly;
use crate::volatile::write_only::VolatileWriteOnly;
use crate::VolatileAddress;

pub struct Builder<Volatile, Addr> {
    volatile_addr: Volatile,
    offset: Option<usize>,
    bits: Option<usize>,
    add_addr: Option<usize>,
    _marker: PhantomData<Addr>,
}


impl<Addr, VolatileAddr> Builder<VolatileAddr, Addr>
    where VolatileAddr: VolatileAddress<Addr>
{
    pub const fn new(addr: VolatileAddr) -> Builder<VolatileAddr, Addr> {
        Self {
            volatile_addr: addr,
            offset: None,
            bits: None,
            add_addr: None,
            _marker: PhantomData,
        }
    }


    pub fn offset(mut self, offset: usize) -> Builder<VolatileAddr, Addr> {
        self.offset = Some(offset);
        self
    }


    pub fn bits(mut self, bits: usize) -> Builder<VolatileAddr, Addr> {
        self.bits = Some(bits);
        self
    }


    pub fn add_addr(mut self, add: usize) -> Builder<VolatileAddr, Addr> {
        self.add_addr = Some(add);
        self
    }


    pub fn build_readonly<Volatile>(self) -> VolatileReadonly<Addr, Volatile>
        where Volatile: Numeric
    {
        VolatileReadonly::new(
            self.new_config::<Volatile>()
        )
    }


    pub fn build_write_only<Volatile>(self) -> VolatileWriteOnly<Addr, Volatile>
        where Volatile: Numeric
    {
        VolatileWriteOnly::new(
            self.new_config::<Volatile>()
        )
    }


    fn new_config<Volatile: Numeric>(self) -> Config<Addr> {
        Config::new_with_options::<Volatile>(
            self.volatile_addr.address(),
            self.offset,
            self.bits,
            self.add_addr,
        )
    }
}


impl<Addr, VolatileAddr> Clone for Builder<VolatileAddr, Addr>
    where VolatileAddr: Clone
{
    fn clone(&self) -> Self {
        Self {
            volatile_addr: self.volatile_addr.clone(),
            offset: self.offset,
            bits: self.bits,
            add_addr: self.add_addr,
            _marker: PhantomData,
        }
    }
}