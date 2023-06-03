use volatile_bits::{VolatileBitsReadable, VolatileBitsWritable};
use volatile_bits_macros::{volatile_bit_field, volatile_bits};

#[volatile_bits(bits = 8)]
struct Bits1(u64);


#[volatile_bits(
offset = 8,
bits = 3
)]
struct Bits2(u64);


#[volatile_bit_field(addr_ty = u64)]
struct Field {
    b1: Bits1,
    b2: Bits2,
}


fn main() {
    let buff: [u64; 1] = [0b11111011_10000000];

    let field = Field::from(buff.as_ptr() as u64);

    assert_eq!(field
                   .b1()
                   .read_volatile(),
               0b10000000
    );


    assert_eq!(field
                   .b2()
                   .read_volatile(),
               0b011
    );
}

