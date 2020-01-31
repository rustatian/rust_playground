#![feature(asm)]

use std::ops::BitAnd;

const SSIZE: isize = 48;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");

    loop {}
}

fn main() {


}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!("
        mov     0x00($0), %rsp
        ret
       "
    :
    : "r"(new)
    :
    : "alignstack" // it will work without this now, will need it later
    );
}
