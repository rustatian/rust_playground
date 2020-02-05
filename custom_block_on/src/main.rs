use std::task::Context;
use std::task::Poll;
use std::thread;
use std::future::Future;

fn main() {}

fn block_on<F: Future>(future: F) -> F::Output {
    pin_utils::pin_mut!(future);
    let thread = thread::current();
    let waker = async_task::waker_fn(move || thread.unpark());
    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }

    }
}