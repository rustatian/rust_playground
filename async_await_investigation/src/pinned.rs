use core::marker::PhantomPinned;

pub struct SelfReferential {
    pub self_ptr: *const Self,
    pub _pin: PhantomPinned,
}