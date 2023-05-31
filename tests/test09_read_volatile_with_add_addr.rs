use volatile_bits::{volatile_bits, VolatileBitsReadable};

#[volatile_bits(type = u8, add = 0x02)]
struct OffsetAddr3Bytes(u64);


fn main() {
    let buff: [u8; 5] = [1, 2, 3, 4, 5];

    let o1 = OffsetAddr3Bytes::new_unchecked(buff.as_ptr() as u64);
    assert_eq!(o1.read_volatile(), 3);
}

