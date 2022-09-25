use foo::bar;

mod foo;
// use foo::Bar;
fn main() {
    bar::Bar::hello();
    foo::do_foo();
    println!("Hello, world!");
}
