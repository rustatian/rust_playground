use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // store the data in the hashmap
    let mut db: HashMap<String, Bytes> = HashMap::new();

    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            Set(cmd) => {
                db.insert(cmd.key().into(), cmd.value().clone());
                Frame::Simple("OK".into())
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}

// get data from tokio::spawn
async fn data() {
    let handle = tokio::spawn(async {
        "return value"
    });

    let out = handle.await.unwrap();
    println!("{}", out);
}