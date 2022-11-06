mod string_concat;
use string_concat::*;
fn main() {
    let a = "Hello".to_string();
    let b = String::from("Hello");
    let c = "World".to_owned();
    let d = c.clone();
    string_concat();
}
