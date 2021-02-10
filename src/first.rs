pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More,
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
         let new_node = Node {
             elem: value,
             next: self.head,
         };

        self.head = Link::More(new_node);
    }
}