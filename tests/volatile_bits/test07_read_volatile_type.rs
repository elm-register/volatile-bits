use volatile_bits::{volatile_bits, VolatileBitsReadable};

#[volatile_bits(offset = 1, type = u8)]
struct Offset1(u64);


fn main() {
    let buff: [u8; 2] = [0b0000_0101, 0b0];

    let o1 = Offset1::from(buff.as_ptr() as u64);

    fn assert_volatile_type<T>(_: T) {
        assert_eq!(core::any::type_name::<T>(), "u8");
    }
    assert_volatile_type(o1.read_volatile());

    assert_eq!(o1.read_volatile(), 0b10);
}

