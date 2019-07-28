use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

#[allow(dead_code)]
type Link = Option<Rc<RefCell<Node>>>;