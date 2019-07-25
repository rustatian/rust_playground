use std::env;

#[allow(dead_code)]
pub fn chapter1_fn() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hi there! {}", n),
        None => panic!("Didn't receive any name ?")
    }
}

#[allow(dead_code)]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[allow(dead_code)]
pub fn increase_score(mut val: u32, how_much: u32) {
    val += how_much;
    println!("You made {} points", val);
}

pub fn doubler() {
    let dbl  = |x| {
        x * 2
    };

    let doubler = |y| y * 2;
    let value = 5;

    let twice = dbl(value);
    println!("{} doubled is {}", value, twice);

    let big_closure = |b, c| {
        let z = b + c;
        z * twice
    };

    let some_numbers = big_closure(1, 2);
    println!("Result from closure: {}", some_numbers);
}