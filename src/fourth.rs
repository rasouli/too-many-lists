use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

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
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {

        let new_head = Node::new(elem);

        match self.head.take() {
            Some(old_head) => {

                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head)
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let next_node = old_head.borrow_mut().next.take();
            match next_node {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }

            // old_head.borrow_mut().elem -> Error: moving out of borrowed content (borrow_mut returns &mut which our elem:T is the thing we are trying to move out)
            // old_head.into_inner().elem -> Error: cannot move out of Rc, which only allows shared reference.
            // Rc::try_unwrap(old_head).unwrap().elem; -> Error: the method `unwrap` exists but the following trait bounds were not satisfied:
            // `Rc<RefCell<fourth::Node<T>>>: Debug`
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics(){
        let mut list = List::new();

        // empty list
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);


        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));


        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}

