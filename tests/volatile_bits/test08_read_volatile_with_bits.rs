use volatile_bits::{volatile_bits, VolatileBitsReadable};

#[volatile_bits(offset = 1, type = u8, bits = 3)]
struct Offset1(u64);


fn main() {
    let buff: [u8; 2] = [0b1111_1101, 0b1];

    let o1 = Offset1::from(buff.as_ptr() as u64);
    assert_eq!(o1.read_volatile(), 0b110);
}

