use mini_redis::{client, Result};
use bytes::Bytes;
use tokio::sync::mpsc;

#[tokio::main]
pub async fn main() {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    let (mut tx, mut rx) = mpsc::channel(32); // 32 is the cap

    let t1 = tokio::spawn(async {
        tx.send("sending from the first channel").await;
    });

    let mut tx2 = tx.clone();
    let t2 = tokio::spawn(async {
        tx2.send("sending from the second channel").await;
    });

    while let Some(message) = rx.recv().await {
        println!("got an message: {}", message);
    }
}

#[derive(Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        val: Bytes,
    },
}