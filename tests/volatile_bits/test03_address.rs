use volatile_bits::volatile_bits;

#[volatile_bits]
struct UsizeBits(usize);

#[volatile_bits]
struct U64Bits(u64);

#[volatile_bits]
struct U32Bits(u32);


fn main() {
    let b1 = UsizeBits::from(0x30);
    assert_eq!(b1.address(), 0x30usize);

    let b2 = U64Bits::from(0x31);
    assert_eq!(b2.address(), 0x31u64);

    let b3 = U32Bits::from(0x32);
    assert_eq!(b3.address(), 0x32u32);
}