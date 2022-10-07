#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct DoubleLinkedList<T> {
    head: Option<Rc<LinkedListNode<T>>>,
}

#[derive(Debug)]
struct LinkedListNode<T> {
    next: Option<Rc<LinkedListNode<T>>>,
    prev: RefCell<Option<Weak<LinkedListNode<T>>>>,
    data: T,
}

impl<T> DoubleLinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn append(&mut self, data: T) -> Self {
        let new_node = Rc::new(LinkedListNode {
            data,
            next: self.head.clone(),
            prev: RefCell::new(None),
        });

        match self.head.clone() {
            Some(node) => {
                let mut prev = node.prev.borrow_mut();
                *prev = Some(Rc::downgrade(&new_node));
            }
            None => {}
        }
        DoubleLinkedList {
            head: Some(new_node),
        }
    }
}

pub fn double_linked_list() {
    let list_of_nums = DoubleLinkedList::new().append(1).append(2).append(3);
    println!("{:#?}", list_of_nums);
}
