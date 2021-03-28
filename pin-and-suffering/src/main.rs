use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let fut = MyFuture {};
    println!("Awaiting fut...");
    fut.await;
    println!("Awaiting fun...done!");
}

struct MyFuture {}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}
