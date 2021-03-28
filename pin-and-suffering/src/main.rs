use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncRead, ReadBuf};
use tokio::time::Sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {}

struct SlowRead<R> {
    reader: Pin<Box<R>>,
}

impl<R> SlowRead<R> {
    fn new(reader: R) -> Self {
        Self {
            reader: Box::pin(reader),
        }
    }
}

impl<R> AsyncRead for SlowRead<R>
where
    R: AsyncRead,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        self.reader.as_mut().poll_read(cx, buf)
    }
}
