use std::io::{ErrorKind, Read};
use std::fs::File;
use std::io;

fn main() {
    // let f = std::fs::File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match std::fs::File::create("hello.txt") {
    //             Ok(fs) => {
    //                 fs
    //             }
    //             Err(err) => {
    //                 panic!("problem creating the file: {:?}", err);
    //             }
    //         },
    //         other_error => {
    //             panic!("problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    let f = std::fs::File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("error creating the file");
            })
        } else {
            panic!("error opening");
        }
    });
}

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s);
    Ok(s)
}