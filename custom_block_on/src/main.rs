use std::task::Context;
use futures::Future;
use futures::task::Poll;
use std::thread;

fn main() {



}

fn block_on<F:Future>(future: F) -> F::Output {
    let waker = todo!();
    let cx = &mut Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(cx) {
           Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}