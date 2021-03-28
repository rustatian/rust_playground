use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("hello");
    sleep(Duration::from_millis(500));
    println!("Goodbye");
}
