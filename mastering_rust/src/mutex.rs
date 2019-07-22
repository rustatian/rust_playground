use std::sync::Mutex;

pub fn mutex_f() {
    let value = Mutex::new(23);
    *value.lock().unwrap() += 1;
    println!("{}", value.lock().unwrap());
} //unlocks here automatically