use volatile_bits::VolatileAddress;
use volatile_bits_macros::volatile_address;

#[volatile_address]
struct U16Address(u16);


#[volatile_address]
struct U32Address(u32);


#[volatile_address]
struct U64Address(u64);


#[volatile_address]
struct UsizeAddress(usize);


fn main() {
    assert_eq!(U16Address::from(0xFF).address(), 0xFF);
    assert_eq!(U32Address::from(0xFF).address(), 0xFF);
    assert_eq!(U64Address::from(0xFF).address(), 0xFF);
    assert_eq!(UsizeAddress::from(0xFF).address(), 0xFF);
}