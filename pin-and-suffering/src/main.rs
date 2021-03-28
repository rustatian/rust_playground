use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let fut = MyFuture::new();
    println!("Awaiting fut...");
    fut.await;
    println!("Awaiting fun...done!");
}

struct MyFuture {
    slept: bool,
}
impl MyFuture {
    fn new() -> Self {
        MyFuture { slept: false }
    }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll()");

        match self.slept {
            false => {
                let waker = cx.waker().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(1000));
                    waker.wake();
                });

                self.slept = true;

                Poll::Pending
            }
            true => Poll::Ready(()),
        }
    }
}
