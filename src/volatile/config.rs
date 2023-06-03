use crate::numeric::Numeric;

#[derive(core::fmt::Debug)]
pub struct Config<Addr> {
    addr: Addr,
    offset: usize,
    bits: usize,
    add_addr: usize,
}


impl<Addr> Config<Addr> {
    pub fn new_with_options<Volatile: Numeric>(
        addr: Addr,
        offset: Option<usize>,
        bits: Option<usize>,
        add_addr: Option<usize>,
    ) -> Config<Addr> {
        Self::new(
            addr,
            offset.unwrap_or(0),
            bits.unwrap_or(Volatile::max_bits()),
            add_addr.unwrap_or(0),
        )
    }


    pub const fn new(
        addr: Addr,
        offset: usize,
        bits: usize,
        add_addr: usize,
    ) -> Config<Addr> {
        Self {
            addr,
            offset,
            bits,
            add_addr,
        }
    }


    pub fn offset(&self) -> usize {
        self.offset
    }


    pub fn bits(&self) -> usize {
        self.bits
    }


    pub fn add_addr(&self) -> usize {
        self.add_addr
    }
}


impl<Addr> Config<Addr> where Addr: Copy {
    pub fn addr(&self) -> Addr {
        self.addr
    }
}


impl<Addr> Clone for Config<Addr> where Addr: Clone {
    fn clone(&self) -> Self {
        Self{
            addr: self.addr.clone(),
            offset: self.offset,
            bits: self.bits,
            add_addr: self.add_addr,
        }
    }
}