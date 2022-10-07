mod linked_list;
use linked_list::*;
mod double_linked_list;
use double_linked_list::*;
use std::rc::Rc;
mod cell;
use cell::*;

fn main() {
    // linked_list();
    // double_linked_list();
    let k = Rc::new(Box::new(5));
    let j = Rc::clone(&k);
    let y = Rc::clone(&k);
    drop(k);
    println!("{}", y);
    println!("{}", Rc::strong_count(&j));
    println!("{:?}", cell_());
}
