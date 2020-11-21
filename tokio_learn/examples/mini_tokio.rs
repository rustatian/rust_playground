use crossbeam::channel;
use futures::task;
use futures::task::ArcWake;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

// type Task = Pin<Box<dyn Future<Output=()> + Send>>;

fn main() {}

struct MiniTokio {
    scheduler: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

struct Task {
    // The `Mutex` is to make `Task` implement `Sync`. Only
    // one thread accesses `future` at any given time. The
    // `Mutex` is not required for correctness. Real Tokio
    // does not use a mutex here, but real Tokio has
    // more lines of code than can fit in a single tutorial
    // page.
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }
    fn poll(self: Arc<Self>) {
        // create a waker from the Task
        // self uses ArcWake impl
        let waker = task::waker(self);
        let mut cx = Context::from_waker(&waker);

        // no other threads tries to lock the future
        let mut future = self.future.try_lock().unwrap();
        // poll the future
        let _ = future.as_mut().poll(&mut cx);
    }
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduler) = channel::unbounded();
        MiniTokio { scheduler, sender }
    }

    // spawn a future onto the mini-tokio instance
    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
    }

    // this approach will burn CPU as we polling every future in the queue
    fn run(&mut self) {
        while let Ok(task) = self.scheduler.recv() {
            task.poll();
        }
    }
}
