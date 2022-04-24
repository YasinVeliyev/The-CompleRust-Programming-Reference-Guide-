mod loops;

pub fn func_lib<T: Fn()>(a: T) {
    a()
}

pub fn b() {
    println!("Hello")
}
