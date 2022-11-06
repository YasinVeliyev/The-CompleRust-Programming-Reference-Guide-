pub fn string_concat() {
    let foo = "Foo";
    let bar = "Bar";
    let baz = foo.to_owned() + bar;
    println!("foo: {} + bar: {} = baz: {}", foo, bar, baz);
}
