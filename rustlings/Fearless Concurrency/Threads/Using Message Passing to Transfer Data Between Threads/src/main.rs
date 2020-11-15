use std::sync::mpsc;
use std::thread;

fn main() {
    // mpsc - multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    thread::spawn(|| {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
