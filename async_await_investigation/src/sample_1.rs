use std::task::{Context, Poll};
use std::pin::Pin;
use futures::future::{self, Either, Future, FutureExt};
struct StringLen<F> {
    inner_future: F,
}

impl<F: Future> Future for StringLen<F> where F: Future<Output=String> {
    type Output = usize;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner_future.poll(cx) {
            Poll::Ready(s) => Poll::Ready(s.len()),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn string_len(string: impl Future<Output=String>) -> impl Future<Output=usize> {
    StringLen {
        inner_future: string,
    }
}

// usage
fn file_len() -> impl Future<Output = usize> {
    let file_content_future = async_read_file("foo.txt");
    string_len(file_content_future)
}

fn async_read_file(name: &str) -> impl Future<Output = String> {
    future::ready(String::from(name))
}