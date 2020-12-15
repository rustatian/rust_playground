use std::sync::mpsc;
use std::sync::mpsc::IntoIter;
use std::thread::spawn;

pub trait OffThreadExt: Iterator {
    /// Transform this iterator into an off-thread iterator: the
    /// `next()` calls happen on a separate worker thread, so the
    /// iterator and the body of your loop run concurrently.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
where
    T: Iterator + Send + Sync + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);

        // Move this iterator to a new worker thread and run it there.
        spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });

        // Return an iterator that pulls values from the channel.
        receiver.into_iter()
    }
}
