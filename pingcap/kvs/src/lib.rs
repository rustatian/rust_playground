#![allow(unused_variables)]
#[allow(unused_mut)]
pub struct KvStore {}

impl KvStore {
    pub fn new() -> Self {
        KvStore {}
    }

    pub fn set(&self, key: String, value: String) {}
    pub fn get(&self, key: String) -> Option<String> {
        None
    }
    pub fn remove(&self, key: String) {}
}