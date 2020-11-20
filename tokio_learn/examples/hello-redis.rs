use mini_redis::{client, Result};
use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};
use std::option::Option::Some;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
pub async fn main() {
    let (mut tx, mut rx) = mpsc::channel(32); // 32 is the cap
    let mut tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".into(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();

        // await the response
        let res = resp_rx.await;
        println!("GOT: {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_s, resp_r) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".into(),
            val: "bar".into(),
            resp: resp_s,
        };
        tx2.send(cmd).await.unwrap();

        let res = resp_r.await;
        println!("GOT: {:?}", res);
    });

    // Open a connection to the mini-redis address.
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}