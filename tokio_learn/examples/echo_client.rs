use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, TcpListener};
use bytes::Bytes;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:6001").await.unwrap();
    loop {
        let (mut stream, addredss) = socket.accept().await.unwrap();
        stream.set_nodelay(true);
        process(stream).await;
    }
}

async fn process(mut stream: TcpStream) {
    //let mut v = vec![];
    let (mut rd, mut wr) = stream.split();

    let mut buf = &mut [0_u8; 17];

    rd.read_exact(&mut buf[..]).await;

    println!("data: {:?}", &buf[..]);
}