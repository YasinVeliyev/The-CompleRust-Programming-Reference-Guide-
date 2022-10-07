#![allow(dead_code)]

use std::rc::Rc;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn append(&self, data: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                data,
                next: self.head.clone(),
            })),
        }
    }
}

pub fn linked_list() {
    let list_of_nums = LinkedList::new().append(1).append(2);
    println!("{:#?}", list_of_nums);
}
