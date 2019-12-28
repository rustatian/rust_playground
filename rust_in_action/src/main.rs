use std::rc::Rc;
use std::sync::{Arc, Mutex};

mod arrays;
mod generics;
mod grep_lite_v1;
mod lang_foundations;

fn main() {
    let n: f64 = 42.42;

    let (signbit, exponent, fraction) = deconstruct_f32(n);

    println!("{} -> [sign:{}, exponent:{}]", n, signbit, exponent);
}

fn deconstruct_f32(n: f64) -> (u64, u64, u64) {
    let n_: u64 = unsafe {
        std::mem::transmute(n)
    };
    

    let sign = (n_ >> 63) & 1;
    let exponent = (n_ >> 52) & 0xff;
    let fraction = 0b00000000_01111111_11111111_11111111_11111111_11111111_11111111_11111111 & n_;

    (sign, exponent, fraction)
}

