#[cfg(test)]
#[test]
fn it_works() {
    let t = trybuild2::TestCases::new();
    t.pass("tests/test01_new_unchecked.rs");
    t.pass("tests/test02_new_unchecked_using_address_type.rs");
    t.pass("tests/test03_address.rs");
    t.pass("tests/test04_read_volatile.rs");
    t.pass("tests/test05_write_volatile.rs");
    t.pass("tests/test06_read_volatile_offset.rs");
    t.pass("tests/test07_read_volatile_type.rs");
    t.pass("tests/test08_read_volatile_with_bits.rs");
    t.pass("tests/test09_read_volatile_with_add_addr.rs");
    t.pass("tests/test10_write_volatile_with_options.rs");
}