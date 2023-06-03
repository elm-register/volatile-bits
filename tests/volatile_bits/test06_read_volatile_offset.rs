use volatile_bits::volatile_bits;
use volatile_bits::VolatileBitsReadable;

#[volatile_bits(offset = 1)]
struct Offset1(u64);


#[volatile_bits(offset = 3)]
struct Offset3(u64);

fn main() {
    let buff: [u64; 2] = [0b0000_0101, 0b0];

    let o1 = Offset1::from(buff.as_ptr() as u64);
    assert_eq!(o1.read_volatile(), 0b10);


    let o3 = Offset3::from(buff.as_ptr() as u64);
    assert_eq!(o3.read_volatile(), 0);
}