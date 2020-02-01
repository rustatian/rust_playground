#![feature(asm)]

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
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    let stack_bottom = unsafe {
        stack.as_mut_ptr().offset(SSIZE)
    };

    let stack_ptr = stack.as_mut_ptr();

    unsafe {
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        #[allow(clippy::cast_ptr_alignment)]
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;

        for i in (0..SSIZE).rev() {
            println!("mem: {}, val: {}", stack_ptr.offset(i as isize) as usize, *stack_ptr.offset(i as isize));
        }

        gt_switch(&ctx);
    }


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
