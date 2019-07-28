use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

#[allow(dead_code)]
pub fn threading_fn() {
    let x = 10;
    // to use x var in the thread, should move it
    let handler = thread::spawn( move ||{
        println!("Hello from a thread, number is: {}", x);
    });

    match handler.join() {
        Ok(_) => { println!("ok") }
        Err(_) => { println!("hukei") }
    }
}
#[allow(dead_code)]
pub fn channels_fn() {
    const N:i32=10;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();

    let handlers = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = _tx.send(i).unwrap();
        })
    });

    for h in handlers {
        h.join().unwrap();
    }


//    loop {
//        println!("{:?}", rx.recv().unwrap())
//    }


    // receive N times
    let numbers: Vec<i32> = (0..N).map(|_|rx.recv().unwrap()).collect();
    println!("{:?}", numbers);
}


pub fn shared_state() {
    use std::thread;
    use std::sync::{Mutex, Arc};
    use std::sync::MutexGuard;

    let v: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));

    let handles = (0..10).map(|i| {
        let numbers: Arc<Mutex<Vec<i32>>> = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector: MutexGuard<Vec<i32>> = numbers.lock().unwrap();
            vector.push(i);
            (*vector).push(i);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *v.lock().unwrap());
    println!("{:?}", v.lock().unwrap());
}







































