use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
    drop(list);

    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, *(y.deref()));

    let m = MyBox::new(String::from("RUST"));
    hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("{}", name);
}

//------------------------------------------------------------------------

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data)
    }
}
