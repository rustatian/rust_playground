use std::future::Future;
use futures::task::{Context, Poll};
use tokio::macros::support::Pin;
use std::time::Instant;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(5);
    let future = Delay { when };
    let out = future.await;
    println!("out: {}", out);
}

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("hello");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

