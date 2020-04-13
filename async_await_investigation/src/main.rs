mod sample_2;
mod pinned;

fn main() {
    let mut heap_value = Box::pin(pinned::SelfReferential {
        self_ptr: std::ptr::null(),
        _pin: core::marker::PhantomPinned,
    });

    let ptr = &*heap_value as *const pinned::SelfReferential;
    heap_value.self_ptr = ptr;

    let stack_value = std::mem::replace(&mut *heap_value, pinned::SelfReferential {
        self_ptr: 0 as *const _,
        _pin: core::marker::PhantomPinned,
    });

    println!("heap value at: {:p}", heap_value);
    println!("internal reference: {:p}", heap_value.self_ptr);
}

struct SelfReferential {
    self_ptr: *const Self,
}