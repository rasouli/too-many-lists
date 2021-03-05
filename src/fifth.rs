use std::mem;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let old_tail = mem::replace(&mut self.tail, Some(new_tail));

        match old_tail {
            Some(mut old_tail) => old_tail.next = Some(new_tail),
            None => self.head = Some(new_tail),
        }
    }
}
