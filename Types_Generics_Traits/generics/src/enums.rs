#[derive(Debug)]
enum Transmission<T> {
    Signal(T),
    NoSignal,
}

pub fn _enums() {
    println!("{:?}", Transmission::Signal(6))
}
