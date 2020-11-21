use tokio::net::TcpStream;
use mini_redis::{Result, Frame};
use bytes::BytesMut;

fn main() {}

struct Connection {
    stream: TcpStream,
    bytes: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream,
            bytes: BytesMut::with_capacity(4096),
        }
    }
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        Ok(None)
    }
    pub async fn write_frame(&mut self) -> Result<()> {
        Ok(())
    }
}