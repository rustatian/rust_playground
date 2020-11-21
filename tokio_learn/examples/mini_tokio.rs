use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::task;

type Task = Pin<Box<dyn Future<Output=()> + Send>>;

fn main() {}

struct MiniTokio {
    tasks: VecDeque<Task>
}

impl MiniTokio {
    fn new() -> Self {
        MiniTokio {
            tasks: VecDeque::new(),
        }
    }

    // spawn a future onto the mini-tokio instance
    fn spawn<F>(&mut self, future: F)
        where
            F: Future<Output=()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(future));
    }

    fn run(&mut self) {
        // create noop waker
        let waker = task::noop_waker();
        // create context from waker
        let mut cx = Context::from_waker(&waker);

        // get task from the vec
        while let Some(mut task) = self.tasks.pop_front() {
            // if task is not ready -> push it back
            if task.as_mut().poll(&mut cx).is_pending() {
                self.tasks.push_back(task);
            }
        }
    }
}