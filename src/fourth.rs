use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(Node {
                elem: elem,
                next: None,
                prev: None,
            }))
    }
}

impl <T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
}