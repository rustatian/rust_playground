use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const PATH: &str = "foo.txt";

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:1111").await.unwrap();

    let (mut rd, mut wr) = socket.split();

    wr.write_all(b"hello\r\n").await?;

    Ok(())
}