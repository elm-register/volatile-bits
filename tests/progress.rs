#[cfg(test)]
#[test]
fn it_works() {
    let t = trybuild2::TestCases::new();
    t.pass("tests/volatile_bits/*.rs");
    t.pass("tests/volatile_bit_field/*.rs");
    t.pass("tests/volatile_address/*.rs");
}