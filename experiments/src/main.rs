#![allow(dead_code)]

use std::collections::HashMap;
use std::io::Write;
use std::iter::Iterator;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut buf: Vec<u8> = vec![];
    let writer: &mut Write = &mut buf;

    <str as ToString>::to_string("foo"); // fully qualified method name
    let line = "foo foo";

    let words: Vec<String> = line.split_whitespace().map(ToString::to_string).collect();

    // array
    let lazy_caterer: [u32; 6] = [1, 2, 3, 4, 5, 6];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    // [type;size]
    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    println!("{:?}", sv);


    let mut s = "fooo".to_string();
    let t = s;
    s = "foo2".to_string();

    println!("{}, {}", s, t);



    let mut table = Table::new();

    table.insert("Foo".to_string(), vec!["many madrigals".to_string()]);
    show(&table);

    // println!("{:?}", table.get(&"foo".to_string())); borrow of moved value if fn show(table: Table);
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn build_vector() -> Vec<i16> {
    let mut vec = vec![];
    vec.push(10);
    vec.push(11);
    vec
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

struct Args {}

impl Iterator for Args {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
