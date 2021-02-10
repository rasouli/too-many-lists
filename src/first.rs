use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List{ head: Link::Empty}
    }

    pub fn push(&mut self, value: i32) {
         let new_node = Box::new(Node {
             elem: value,
             next: mem::replace(&mut self.head, Link::Empty),
         });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match &self.head {
            Link::Empty => None,
            Link::More(ref node) => {
                mem::replace(&mut self.head, node.next);
                Some(node.elem)
            }
        }
    }
}