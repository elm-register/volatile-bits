use crate::numeric::Numeric;
use crate::readonly::VolatileBitsReadonly;

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


    pub fn build_readonly<Volatile>(self) -> VolatileBitsReadonly<Addr, Volatile>
        where Volatile: Numeric
    {
        VolatileBitsReadonly::new(
            self.addr,
            self.offset.unwrap_or(0),
            self.bits.unwrap_or(Volatile::max_bits()),
            self.add_addr.unwrap_or(0),
        )
    }
}