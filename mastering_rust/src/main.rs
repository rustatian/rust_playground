use crate::threading::shared_state;
use crate::tokio::tokio_start;

mod mutex;
mod chapter1;
mod strings;
mod threading;
mod interior_mutability;
mod tokio;

#[allow(dead_code)]
fn main() {
//    let my_closure = || ();
//    let score: u32 = 2048;
//    increase_score(score, 30);
//
//    stringsss();
//    threading_fn();
//
//    let ss = String::from("wha tever");
//
//
//    println!("{}", sanitize(ss))
    tokio_start();
}


