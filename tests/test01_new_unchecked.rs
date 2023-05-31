use volatile_bits_impl::volatile_bits;

#[volatile_bits]
struct Bits(u64);


fn main() {
    let _bits = Bits::new_unchecked(0xFFu64);
}