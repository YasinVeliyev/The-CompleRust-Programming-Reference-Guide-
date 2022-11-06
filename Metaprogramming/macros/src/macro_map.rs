#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($k,$v);)*
        map
    }};
}

pub fn macro_map() {
    let m = map!("a"=>5,"b"=>6);
    println!("{:?}", m);
}
