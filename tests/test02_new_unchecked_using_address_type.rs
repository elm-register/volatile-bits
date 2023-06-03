use volatile_bits::volatile_bits;

#[volatile_bits]
struct UsizeBits(usize);

#[volatile_bits]
struct U32Bits(u32);


fn main() {
    let _ = UsizeBits::from(0xFFusize);
    let _ = U32Bits::from(0xFFu32);
}