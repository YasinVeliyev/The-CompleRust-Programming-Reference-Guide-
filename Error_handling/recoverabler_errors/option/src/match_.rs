use std::collections::HashMap;

pub fn match_() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    let value = map.get("one");
    let incremented_value = match value {
        Some(val) => val + 1,
        None => 0,
    };
    println!("With match: {}", incremented_value);
}
