use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;


fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(strm) => {
                thread::spawn(|| {
                    handle_connection(strm);
                });
            }
            Err(e) => {
                println!("couldn't get client: {:?}", e);
            }
        }
//        let stream = stream.unwrap();
//        thread::spawn(|| {
//            handle_connection(stream);
//        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response).unwrap();
    stream.flush().unwrap();
}
