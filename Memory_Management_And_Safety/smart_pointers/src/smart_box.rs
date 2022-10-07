fn box_ref<T>(b: T) -> Box<T> {
    let a = b;
    Box::new(a)
}
struct Foo;

pub fn box_() {
    let boxed_one = Box::new(Foo);
    let unboxed_one = *boxed_one;
    box_ref(unboxed_one);
    println!("Hello, world!");
}
