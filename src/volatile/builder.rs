use core::marker::PhantomData;

use crate::numeric::Numeric;
use crate::volatile::config::Config;
use crate::volatile::readonly::VolatileReadonly;
use crate::volatile::write_only::VolatileWriteOnly;
use crate::volatile::Volatile;
use crate::VolatileAddress;

#[derive(Debug)]
pub struct Builder<Addr, Value> {
    volatile_addr: Value,
    offset: Option<usize>,
    bits: Option<usize>,
    add_addr: Option<usize>,
    _marker: PhantomData<Addr>,
}

impl<Addr, VolatileAddr> Builder<Addr, VolatileAddr>
where
    VolatileAddr: VolatileAddress<Addr> + Clone,
    Addr: Clone,
{
    pub const fn new(addr: VolatileAddr) -> Builder<Addr, VolatileAddr> {
        Self {
            volatile_addr: addr,
            offset: None,
            bits: None,
            add_addr: None,
            _marker: PhantomData,
        }
    }

    pub fn offset(mut self, offset: usize) -> Builder<Addr, VolatileAddr> {
        self.offset = Some(offset);
        self
    }

    pub fn bits(mut self, bits: usize) -> Builder<Addr, VolatileAddr> {
        self.bits = Some(bits);
        self
    }

    pub fn add_addr(mut self, add: usize) -> Builder<Addr, VolatileAddr> {
        self.add_addr = Some(add);
        self
    }

    pub fn build_readonly_type_as<Value>(self) -> VolatileReadonly<Addr, Value>
    where
        Value: Numeric,
    {
        VolatileReadonly::new(self.new_config::<Value>())
    }

    pub fn build_write_only_type_as<Value>(self) -> VolatileWriteOnly<Addr, Value>
    where
        Value: Numeric,
    {
        VolatileWriteOnly::new(self.new_config::<Value>())
    }

    pub fn build_type_as<Value>(self) -> Volatile<Addr, Value>
    where
        Value: Numeric,
    {
        Volatile::new(
            VolatileReadonly::new(self.new_config::<Value>()),
            VolatileWriteOnly::new(self.new_config::<Value>()),
        )
    }

    fn new_config<Value: Numeric>(&self) -> Config<Addr> {
        Config::new_with_options::<Value>(
            self.volatile_addr.address(),
            self.offset,
            self.bits,
            self.add_addr,
        )
    }
}
