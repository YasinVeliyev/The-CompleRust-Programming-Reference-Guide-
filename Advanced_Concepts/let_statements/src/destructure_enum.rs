enum Container {
    Item(u64),
    Empty,
}

pub fn destructure_enum() {
    let maybe_item = Container::Item(0);
    let has_item = if let Container::Item(a) = maybe_item {
        println!("{}", a);
        true
    } else {
        false
    };
}
