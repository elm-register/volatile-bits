use volatile_bits::volatile_bits;
use volatile_bits::VolatileBitsWritable;

#[volatile_bits(
type = u8,
bits = 3,
offset = 2,
add = 0x01
)]
struct Bits(u64);


fn main() {
    let buff: [u8; 3] = [1, 0b1111_1111, 3];
    let b = Bits::from(buff.as_ptr() as u64);

    b.write_volatile(0b000).unwrap();
}