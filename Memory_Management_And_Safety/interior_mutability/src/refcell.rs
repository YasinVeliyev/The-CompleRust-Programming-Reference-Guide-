use std::cell::RefCell;

#[derive(Debug)]
struct Bag {
    item: Box<u32>,
}

impl Bag {
    fn new(item: u32) -> Self {
        Self {
            item: Box::new(item),
        }
    }
}

pub fn refcell() {
    let bag = RefCell::new(Bag::new(1));
    let hand1 = &bag;
    let hand2 = &bag;
    *hand1.borrow_mut() = Bag::new(2);
    println!("After set hand1 {:?}  {:?}, {:?}", hand1, bag, hand2);
    *hand2.borrow_mut() = Bag::new(5);
    println!("After set hand2 {:?}  {:?}", hand1, hand2);
}
