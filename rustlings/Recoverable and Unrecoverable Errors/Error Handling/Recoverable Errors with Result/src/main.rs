use std::io::{ErrorKind};
use std::fs::File;

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
