#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::{Arbitrary, Gen};
use rand::Rng;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Clone, Debug)]
struct Positive {
    value: i32,
}

impl Arbitrary for Positive {
    fn arbitrary<G: Gen>(g: &mut G) -> Positive {
        Positive {
            value: g.gen_range(1, i32::max_value() / 2),
        }
    }
}

#[quickcheck]
fn sum_is_greater_than_parts(pos_a: Positive, pos_b: Positive) -> bool {
    let a = pos_a.value;
    let b = pos_b.value;

    let sum = add(a, b);

    sum > a && sum > b
}
