use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:1111").await.unwrap();

    loop {
        let (mut socket, _) = socket.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();

            let mut v = vec![];
            rd.read_to_end(&mut v).await?;
            println!("data: {:?}", v);

            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }

            Ok::<_, io::Error>(())
        });
    }
}