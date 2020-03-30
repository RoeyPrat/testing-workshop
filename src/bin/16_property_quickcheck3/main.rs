#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::TestResult;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[quickcheck]
fn sum_is_commutative(a: i32, b: i32) -> bool {
    add(a, b) == add(b, a)
}

#[quickcheck]
fn sum_is_associative(a: i32, b: i32, c: i32) -> bool {
    add(add(a, b), c) == add(a, add(b, c))
}

#[quickcheck]
fn sum_stays_same_when_adding_zero(a: i32) -> bool {
    add(a, 0) == a
}

#[quickcheck]
fn sum_is_greater_than_parts(a: i32, b: i32) -> TestResult {
    let sum = add(a, b);

    if a <= 0 || b <= 0 {
        return TestResult::discard();
    }

    TestResult::from_bool(sum > a && sum > b)
}
