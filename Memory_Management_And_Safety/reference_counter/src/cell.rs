use std::cell::Cell;

#[derive(Debug)]
struct Bag {
    item: Box<u32>,
}

pub fn cell_() {
    let mut bag = Cell::new(Bag { item: Box::new(32) });
    let hand1 = &mut bag;
    let hand2 = &mut bag;
    *hand1 = Cell::new(Bag { item: Box::new(2) });
    *hand2 = Cell::new(Bag { item: Box::new(22) });
}
