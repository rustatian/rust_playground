use crossbeam::channel;
use futures::task;
use futures::task::ArcWake;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Instant;
use tokio::time::Duration;
use std::thread;


#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_secs(5);
    let mut delay = Some(Delay { waker: None, when });

    futures::future::poll_fn(move |cx| {
        let mut delay = delay.take().unwrap();
        let res = Pin::new(&mut delay).poll(cx);
        assert!(res.is_pending());

        tokio::spawn(async move {
            delay.await;
        });

        Poll::Ready(())
    }).await;
}

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
    future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn schedule(self: &Arc<Self>) {
        let _ = self.executor.send(self.clone());
    }
    fn poll(self: Arc<Self>) {
        // create a waker from the Task
        // self uses ArcWake impl
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        // no other threads tries to lock the future
        let mut future = self.future.try_lock().unwrap();
        // poll the future
        let _ = future.as_mut().poll(&mut cx);
    }
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
        where
            F: Future<Output=()> + Send + 'static,
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
            F: Future<Output=()> + Send + 'static,
    {}

    // this approach will burn CPU as we polling every future in the queue
    fn run(&mut self) {
        while let Ok(task) = self.scheduler.recv() {
            task.poll();
        }
    }
}


struct Delay {
    waker: Option<Arc<Mutex<Waker>>>,
    when: Instant,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
