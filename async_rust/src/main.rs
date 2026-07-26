mod thread;
use std::arch::asm;

use crate::thread::start_thread;

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t;
    let x = deref(t_ptr);
    println!("x = {}", x);

    start_thread();
}

fn deref(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe {
        asm!(
            "mov {0}, [{1}]",
            out(reg) res,
            in(reg) ptr
        );
    }
    res
}
