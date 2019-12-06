use std::rc::Rc;
use std::sync::{Arc, Mutex};

mod lang_foundations;

fn main() {
    lang_foundations::search_match();
}

fn types_declaring() {
    let a = 10;
    let b = Box::new(20);
    let c = Rc::new(Box::new(30));
    let d = Arc::new(Mutex::new(40));

    let res = add_with_lifetimes(&10, &20);
}

pub fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    5
}
