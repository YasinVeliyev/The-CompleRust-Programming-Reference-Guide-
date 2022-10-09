use std::fs::File;
use std::io::Read;
use std::path::Path;
fn main() {
    let path = Path::new("./src/msain.rs");
    let file = File::open(path);
    let mut s = String::new();
    match file {
        Ok(mut file) => {
            file.read_to_string(&mut s);
        }
        Err(err) => println!("{:?}", err),
    };
    println!("{}", s);
}
