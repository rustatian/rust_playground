use crate::mutex::mutex_f;
use crate::chapter1::{chapter1_fn, increase_score, doubler};
use crate::strings::stringsss;


mod mutex;
mod chapter1;
mod strings;

#[allow(dead_code)]
fn main() {
    let my_closure = || ();
    let score: u32 = 2048;
    increase_score(score, 30);

    stringsss();
}


