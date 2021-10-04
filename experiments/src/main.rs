#![allow(dead_code)]
use std::io::Write;
use std::iter::Iterator;

fn main() {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut Write = &mut buf;

    <str as ToString>::to_string("foo"); // fully qualified method name
    let line = "foo foo";

    let words: Vec<String> = line.split_whitespace().map(ToString::to_string).collect();
}

trait Visible {
    fn hit_set(&self, x: i32, y: i32) -> bool;
}

pub trait Iterator2 {
    type Item; // associated type

    fn next(&mut self) -> Option<Self::Item>;
}

fn collect_into_vector<I: std::iter::Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();

    for value in iter {
        results.push(value);
    }
    results
}
