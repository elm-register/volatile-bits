use crate::numeric::Numeric;
use crate::volatile::config::Config;
use crate::volatile::readonly::VolatileReadonly;
use crate::volatile::write_only::VolatileWriteOnly;

pub struct Builder<Addr> {
    addr: Addr,
    offset: Option<usize>,
    bits: Option<usize>,
    add_addr: Option<usize>,
}


impl<Addr> Builder<Addr> {
    pub const fn new(addr: Addr) -> Builder<Addr> {
        Self {
            addr,
            offset: None,
            bits: None,
            add_addr: None,
        }
    }


    pub fn offset(mut self, offset: usize) -> Builder<Addr> {
        self.offset = Some(offset);
        self
    }


    pub fn bits(mut self, bits: usize) -> Builder<Addr> {
        self.bits = Some(bits);
        self
    }


    pub fn add_addr(mut self, add: usize) -> Builder<Addr> {
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
            self.addr,
            self.offset,
            self.bits,
            self.add_addr,
        )
    }
}


impl<Addr: Clone> Clone for Builder<Addr> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr.clone(),
            offset: self.offset,
            bits: self.bits,
            add_addr: self.add_addr,
        }
    }
}