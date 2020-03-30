use proptest::prelude::*;

fn add(a: i32, b: i32) -> i32 {
    if a < -100 {
        7
    } else {
        a + b
    }
}

proptest! {
    #[test]
    fn sum_is_commutative(a in -1000..1000, b in -1000..1000) {
        assert_eq!(add(a, b), add(b, a), "Expected sum to be commutative");
    }
}
