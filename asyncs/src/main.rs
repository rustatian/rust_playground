use {
    futures::{
        future::{FutureExt, BoxFuture},
        task::{ArcWake, waker_ref},
    },
    std::{
        future::Future,
        sync::{Arc, Mutex},
        sync::mpsc::{sync_channel, SyncSender, Receiver},
        task::{Context, Poll},
        time::Duration,
    },
    // The timer we wrote in the previous section:
    timer_future::TimerFuture,
};

mod timer_future;

fn main() {
    println!("Hello, world!");
}
