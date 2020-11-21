use tokio::net::TcpStream;
use mini_redis::{Result, Frame};
use bytes::BytesMut;
use tokio::io::AsyncReadExt;

fn main() {}

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream,
            buffer: BytesMut::with_capacity(4096),
        }
    }
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // parse frame
            if let Some(frame) = self.parse_frame? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            if self.stream.read_buf(&mut self.buffer).await? == 0 {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("connection reset by peer".into())
                };
            }
        }
    }
    pub async fn write_frame(&mut self) -> Result<()> {
        Ok(())
    }
}