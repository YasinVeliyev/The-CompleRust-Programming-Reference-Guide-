use std::collections::HashMap;

pub fn if_let() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    let value = map.get("one");
    let incremented_value = if let Some(val) = value { val + 1 } else { 0 };
    println!("With if_let: {}", incremented_value);
}
