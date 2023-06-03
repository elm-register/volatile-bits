use volatile_bits::volatile_bits;

#[volatile_bits]
struct Bits(u64);


fn main() {
    let _bits = Bits::from(0xFFu64);
}