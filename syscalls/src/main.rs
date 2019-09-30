#![feature(asm)]

fn main() {
    unsafe {
        println!("{}", syscall0(10));
    }
}

pub unsafe fn syscall0(n: usize) -> usize {
    let ret : usize;
    asm!("syscall" : "={rax}"(ret)
                   : "{rax}"(n)
                   : "rcx", "r11", "memory"
                   : "volatile");
    ret
}