use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use http_thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        pool.execute(move || {
            handle_connection(stream.unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    let _num = stream.write(response).unwrap();
    stream.flush().unwrap();
}