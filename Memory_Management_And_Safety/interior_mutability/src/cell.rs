use std::cell::Cell;

#[derive(Debug, Copy, Clone)]
struct Bag {
    item: u32,
}

impl Bag {
    fn new(item: u32) -> Self {
        Self { item }
    }
}

pub fn cell() {
    let bag = Cell::new(Bag::new(5));
    let hand1 = &bag;
    let hand2 = &bag;
    hand1.set(Bag::new(7));
    println!(
        "After set hand1 {:?}  {:?}, {:?}",
        hand1.get(),
        bag.get(),
        hand2.get()
    );
    hand2.set(Bag::new(8));
    println!(
        "After set hand2 {:?}  {:?}  {:?}",
        hand1.get(),
        bag.get(),
        hand2.get()
    )
}
