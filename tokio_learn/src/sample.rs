use std::sync::Mutex;

struct CanIncrement {
    mutex: Mutex<i32>,
}

impl CanIncrement {
    // not marked as async
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn incr_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    just_hello().await;
}

async fn just_hello() {
    println!("Hello");
}