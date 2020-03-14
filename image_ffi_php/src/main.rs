use std::thread;

fn main() {
    let builder = thread::Builder::new().stack_size(10000 * 1024 * 1024); // 1Gb of stack space

    // create a handler
    let handler = builder.spawn(|| println!("{}", fib(50_000_000))).unwrap();

    handler.join().unwrap();
}

fn fib(n: i32) -> i32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
