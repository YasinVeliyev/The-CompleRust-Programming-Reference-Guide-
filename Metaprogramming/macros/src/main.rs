mod first_macro;
use first_macro::*;
mod macro_map;
use macro_map::*;
mod http_tester;
use futures::executor::block_on;
use http_tester::*;

fn main() {
    println!("Hello, world!");
    first_macro();
    // println!("{}", scanline!());
    let v = vec![5 + 5; 0];
    println!("{:?}", v);
    macro_map();
    http_test();
}
