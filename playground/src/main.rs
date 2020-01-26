#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;
use crate::ConnectivityCheck::{Pong, Ping, Pang};

mod svg_sample;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    two_way_communication();
}

fn two_way_communication() {
    let n_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Pong => {
                eprintln!("unexpected pong response")
            },
            Ping => {
                responses_tx.send(Pong).unwrap();
            },
            Pang => {
                return;
            },
        }
    });

    for _ in 0..n_messages {
        requests_tx.send(Ping).unwrap();
    }

    requests_tx.send(Pang).unwrap();

    for _ in 0..n_messages {
        select! {
            recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }
}

fn _receive_sample() {
    let (tx, rx) = unbounded();
    println!("{}", 3);

    thread::spawn(move || {
        tx.send(42);
    });

    select! {
        recv(rx) -> msg => println!("{:?}", msg),
    }
}