use futures::Future;
use std::pin::Pin;
use futures::channel::oneshot;
use std::sync::atomic::AtomicUsize;
use std::sync::{Mutex, Arc};
use once_cell::sync::Lazy;
use crossbeam::channel;
use std::thread;
use futures::task::Context;

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
    // The future is pinned to the HEAP, because only PINNED futures can be polled
    // MUTEX: every waker associated with the task will hold a `Task` reference so that it can wake the task
    // by pushing it into global task queue. `Task` instances are shared among threads, but polling
    // the future requires mutable access to it. So, MUTEX is here to help :P
    future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>,
}

impl Task {
    fn run(self: Arc<Task>) {
        let waker = todo!();

        let context = &mut Context::from_waker(&waker);
        self.future.try_lock().unwrap().as_mut().poll(context);
    }
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

    let task = Arc::new(Task {
        state: AtomicUsize::new(0),
        future: Mutex::new(Box::pin(future)),
    });


    QUEUE.send(task).unwrap();

    Box::pin(async {
        r.await.unwrap()
    })
}


static QUEUE: Lazy<channel::Sender<Arc<Task>>> = Lazy::new(|| {
    let (sender, receiver) = channel::unbounded::<Arc<Task>>();

    for _ in 0..num_cpus::get().max(1) {
        let receiver = receiver.clone();
        thread::spawn(move || {
            receiver.iter().for_each(|task| task.run())
        });
    }

    sender
});