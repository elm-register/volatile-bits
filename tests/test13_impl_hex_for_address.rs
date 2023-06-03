use volatile_bits_macros::volatile_address;

#[volatile_address]
struct Address(u64);


fn main() {
    let address = Address::from(0xFF);

    assert_eq!(format!("{:x}", address), "ff");
    assert_eq!(format!("{:X}", address), "FF");
}