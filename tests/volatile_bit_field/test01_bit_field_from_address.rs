use volatile_bits_macros::{volatile_address, volatile_bit_field, volatile_bits};

#[volatile_address]
struct Address(u64);


#[volatile_bits(bits = 8)]
struct Bits1(Address);


#[volatile_bits(
offset = 8
)]
struct Bits2(Address);


#[volatile_bit_field(addr_ty = Address)]
struct Field1 {
    b1: Bits1,
}


#[volatile_bit_field(addr_ty = Address)]
struct Field2 {
    b1: Bits1,
    b2: Bits2,
}


fn main() {
    let buff: [u64; 1] = [0b10000000_10000000];
    let _ = Field1::from(Address::from(buff.as_ptr() as u64));
    let _ = Field2::from(Address::from(buff.as_ptr() as u64));
}

