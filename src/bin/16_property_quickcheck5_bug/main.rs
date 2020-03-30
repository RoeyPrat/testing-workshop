#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::{Arbitrary, Gen, TestResult};

#[derive(Clone, Copy, PartialOrd, PartialEq, Debug)]
struct Num {
    value: i32,
}

impl Arbitrary for Num {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Num { value: i32::arbitrary(g) }
    }
}

fn add(a: Num, b: Num) -> Num {
    if a.value <= -1000 {
        Num { value: 7 }
    } else {
        Num {
            value: a.value + b.value,
        }
    }
}

#[quickcheck]
fn sum_is_commutative(a: Num, b: Num) -> bool {
    add(a, b) == add(b, a)
}

#[quickcheck]
fn sum_is_associative(a: Num, b: Num, c: Num) -> bool {
    add(add(a, b), c) == add(a, add(b, c))
}

#[quickcheck]
fn sum_stays_same_when_adding_zero(a: Num) -> bool {
    add(a, Num { value: 0 }) == a
}

#[quickcheck]
fn sum_is_greater_than_parts(a: Num, b: Num) -> TestResult {
    let sum = add(a, b);

    if a.value <= 0 || b.value <= 0 {
        return TestResult::discard();
    }

    TestResult::from_bool(sum > a && sum > b)
}
