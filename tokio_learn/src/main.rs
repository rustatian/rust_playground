use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

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
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // respond with the error
        let response = Frame::Error("unimplemented".into());
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