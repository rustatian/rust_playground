use std::env;

pub fn chapter1_fn() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hi there! {}", n),
        None => panic!("Didn't receive any name ?")
    }
}