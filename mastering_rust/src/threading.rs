use std::thread;


pub fn threading_fn() {
    let handler = thread::spawn(|| {
        println!("Hello from a thread");
    });

    match handler.join() {
        Ok(_) => {println!("ok")},
        Err(_) => {println!("hukei")}
    }
}