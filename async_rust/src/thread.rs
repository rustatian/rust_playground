use std::{process::Output, thread::sleep};

pub fn start_thread() {
    let t1 = std::thread::spawn(move || {
        sleep(std::time::Duration::from_millis(200));
        println!("200 ms task finished");
    });

    let t2 = std::thread::spawn(move || {
        sleep(std::time::Duration::from_millis(100));
        println!("callbacks");

        let t3 = std::thread::spawn(move || {
            sleep(std::time::Duration::from_millis(50));
            println!("50 ms task finished");
        });
        t3.join().unwrap();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn run() -> impl std::future::Future<Output = ()> {
    async {
        sleep(std::time::Duration::from_millis(2000));
    }
}