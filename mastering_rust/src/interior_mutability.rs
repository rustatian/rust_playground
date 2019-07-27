use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: String,
    next: Link,
    prev: Link,
}

type Link = Option<Rc<RefCell<Node>>>;