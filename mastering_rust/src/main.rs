use crate::mutex::mutex_f;
use crate::chapter1::{chapter1_fn, increase_score, doubler};
use crate::strings::stringsss;
use crate::threading::threading_fn;


mod mutex;
mod chapter1;
mod strings;
mod threading;

#[allow(dead_code)]
fn main() {
    let my_closure = || ();
    let score: u32 = 2048;
    increase_score(score, 30);

    stringsss();
    threading_fn();
}


