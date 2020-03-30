fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_sum_is_commutative() {
    let a = 3;
    let b = 5;

    assert_eq!(add(a, b), add(b, a), "Expected sum to be commutative");
}

#[test]
fn test_sum_is_associative() {
    let a = 3;
    let b = 5;
    let c = 5;

    assert_eq!(
        add(add(a, b), c),
        add(a, add(b, c)),
        "Expected sum to be associative",
    );
}

#[test]
fn test_sum_stays_same_when_adding_zero() {
    let a = 3;

    assert_eq!(
        add(a, 0),
        a,
        "Expected sum to stay the same when adding zero"
    );
}

#[test]
fn test_sum_is_greater_than_parts() {
    let a = 3;
    let b = 5;

    let sum = add(a, b);

    assert!(sum > a && sum > b, "Expected sum to be greater than parts");
}
