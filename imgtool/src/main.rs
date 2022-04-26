use std::env;
use std::path::Path;

fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    let rotaded = img.rotate90();
    rotaded.save(path).unwrap();
}
