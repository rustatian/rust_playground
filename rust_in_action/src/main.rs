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
