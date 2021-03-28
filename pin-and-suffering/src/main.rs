use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::Sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let fut = MyFuture::new();
    println!("Awaiting fut...");
    fut.await;
    println!("Awaiting fun...done!");
}

struct MyFuture {
    sleep: Pin<Box<Sleep>>,
}

impl MyFuture {
    fn new() -> Self {
        Self {
            sleep: Box::pin(tokio::time::sleep(Duration::from_secs(1))),
        }
    }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("MyFuture::poll()");
        // self.sleep.as_mut().poll(cx)
        self.sleep.poll_unpin(cx)
    }
}
