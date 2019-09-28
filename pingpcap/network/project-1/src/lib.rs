use std::collections::HashMap;
use std::ops::Deref;

pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            data: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        match self.data.get(&key) {
            Some(val) => Some(val.parse::<String>().unwrap()),
            _ => None,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        match self.data.get(&key) {
            Some(val) => {
                self.data.remove(&key);
            }
            _ => {}
        }
    }
}
