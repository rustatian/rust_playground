use std::rc::Rc;
use std::sync::{Arc, Mutex};

mod arrays;
mod generics;
mod grep_lite_v1;
mod lang_foundations;

fn main() {
    grep_lite_v1::using_regexp();
}

fn types_declaring() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));
}

pub fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    5
}


fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
    let n_:u32 = unsafe {
        std::mem::transmute(n)
    };

    let sign = (n_ >> 31) & 1;
    let exponent = (n_ >> 32) & 0xff;
    let fraction = 0b00000000_01111111_11111111_11111111 & n_;

    (sign, exponent, fraction)
}