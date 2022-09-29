mod functions;
use functions::*;
mod structs;
use structs::*;
mod enums;
use enums::*;
fn main() {
    println!("Hello, world!");
    _func();
    _structs();
    _enums();
    let v = Vec::<u32>::new();
}
