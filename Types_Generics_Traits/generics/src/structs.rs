#[derive(Debug)]
struct GenericStruct<T>(T);

#[derive(Debug)]
struct Container<T> {
    _item: T,
}

impl<T>  Container<T> {
    fn new(item: T) -> Self {
        Self { _item: item }
    }
}

pub fn _structs() {
    let generic = GenericStruct(36);
    let container = Container { _item: "Apple" };
    println!("{:?} {:?}", generic, container);
}
