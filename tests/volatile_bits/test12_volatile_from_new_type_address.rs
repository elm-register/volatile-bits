use volatile_bits::volatile_bits;
use volatile_bits::VolatileBitsWritable;
use volatile_bits_macros::volatile_address;

#[volatile_address]
struct Address(u64);


#[volatile_bits(
type = u8,
bits = 3,
offset = 2,
add = 0x01
)]
struct Bits(Address);


fn main() {
    let buff: [u8; 3] = [1, 0b1111_1111, 3];
    let b = Bits::from(Address::from(buff.as_ptr() as u64));

    b.write_volatile(0b000).unwrap();
}