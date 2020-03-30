fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_sum_is_commutative() {
    let a = 3;
    let b = 5;

    assert_eq!(add(a, b), add(b, a), "Expected sum to be commutative");
}
