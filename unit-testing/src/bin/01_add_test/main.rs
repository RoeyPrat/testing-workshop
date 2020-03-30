fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    let a = 3;
    let b = 5;

    assert_eq!(add(a, b), a + b);
}
