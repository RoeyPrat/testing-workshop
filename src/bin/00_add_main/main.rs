fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 3;
    let b = 5;

    println!("{} + {} = {}", a, b, add(a, b));
}
