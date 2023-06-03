use volatile_bits::volatile_bits;
use volatile_bits::VolatileBitsReadable;

#[volatile_bits]
struct Bits(u64);

fn main() {
    let buff = [0x21u64; 1];

    let b = Bits::from(buff.as_ptr() as u64);
    assert_eq!(b.read_volatile(), 0x21);
}