use std::collections::HashMap;

pub fn unrapping() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    let incremented_value = map.get("one").unwrap_or(&0) + 1;
    let incremented_value = map.get("three").unwrap_or(&0) + 1;
    println!("With unwrapp: {}", incremented_value);
}
