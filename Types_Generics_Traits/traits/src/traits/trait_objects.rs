use std::fmt::Debug;

#[derive(Debug)]
struct Square(f32);

#[derive(Debug)]
struct Rectangle(f32, f32);

trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

pub fn _objects() {
    let shapes: Vec<&dyn Area> = vec![&Square(3.0), &Rectangle(4.0, 2.0)];
    println!("{}", shapes[0].get_area());
}
