use futures::Future;
use std::pin::Pin;
use futures::channel::oneshot;
use std::sync::atomic::AtomicUsize;
use std::sync::Mutex;

fn main() {
    futures::executor::block_on(async {
        let handle = spawn(async { 1 + 2 });
        assert_eq!(handle.await, 3);
    });
}

// task consist of future and it's state
// we need to know whether the task is scheduled for running
struct Task {
    state: AtomicUsize,
    // Output is () because the `spawn()` function has wrapped the original
    // future into one that sends the output into the oneshot channel and then
    // simply returns with `()`
    future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>,
}

// temporary
type JoinHandle<R> = Pin<Box<dyn Future<Output=R> + Send>>;

fn spawn<F, R>(future: F) -> JoinHandle<R> where
    F: Future<Output=R> + Send + 'static,
    R: Send + 'static
{
    let (s, r) = oneshot::channel();
    let future = async move {
        let _ = s.send(future.await);
    };

    todo!();

    Box::pin(async {
        r.await.unwrap()
    })
}
