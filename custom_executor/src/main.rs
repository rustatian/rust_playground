use futures::Future;
use async_task::JoinHandle;

fn main() {
    futures::executor::block_on(async {
        let handle = spawn(async { 1 + 2 });
        assert_eq!(handle.await, 3);
    });
}

fn spawn<F, R>(future: F) -> JoinHandle<R> where
    F: Future<Output=R> + Send + 'static,
    R: Send + 'static
{
    todo!()
}
