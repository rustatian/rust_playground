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



enum MainFuture {
    // initialized
    State0,
    //waiting on delay, future.await line
    State1(Delay),
    // terminated
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use MainFuture::*;
        loop {
            match *self {
                State0 => {
                    let when = Instant::now() + Duration::from_secs(1);
                    let future = Delay { when };
                    *self = State1(future);
                }
                State1(ref mut my_future) => {
                    return match Pin::new(my_future).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = Terminated;
                            Poll::Ready(())
                        }
                        Poll::Pending => {
                            Poll::Pending
                        }
                    };
                }
                Terminated => {
                    panic!("future polled after competition");
                }
            }
        }
    }
}

