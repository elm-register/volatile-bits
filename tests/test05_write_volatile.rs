use volatile_bits::{VolatileBitsReadable, VolatileBitsWritable};
use volatile_bits::volatile_bits;

#[volatile_bits]
struct Bits(u64);

fn main() {
    let buff = [0x21u64; 1];

    let b = Bits::new_unchecked(buff.as_ptr() as u64);
    b.write_volatile(0x30).unwrap();
    assert_eq!(b.read_volatile(), 0x30);
}