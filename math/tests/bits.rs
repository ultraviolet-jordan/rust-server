use math::bits::Bits;

#[test]
fn test_bitcount() {
    let count = Bits::bitcount(15); // 15 in binary is 1111, so count is 4
    assert_eq!(count, 4);
}

#[test]
fn test_setbit_range() {
    let bits = Bits::new();
    let result = bits.setbit_range(0, 1, 3); // Sets bits 1 to 3 in 0, resulting in 14 (1110 in binary)
    assert_eq!(result, 14);
}

#[test]
fn test_setbit_range_toint() {
    let bits = Bits::new();
    let result = bits.setbit_range_toint(0, 3, 1, 3); // Sets bits 1 to 3 to match 3, resulting in 6
    assert_eq!(result, 6);
}

#[test]
fn test_clearbit_range() {
    let bits = Bits::new();
    let result = bits.clearbit_range(15, 1, 3); // Clears bits 1 to 3 in 15, resulting in 1
    assert_eq!(result, 1);
}
