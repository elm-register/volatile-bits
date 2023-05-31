use volatile_bits_impl::volatile_bits;

#[volatile_bits]
struct UsizeBits(usize);

#[volatile_bits]
struct U32Bits(u32);


fn main() {
    UsizeBits::new_unchecked(0xFFusize);
    U32Bits::new_unchecked(0xFFu32);
}