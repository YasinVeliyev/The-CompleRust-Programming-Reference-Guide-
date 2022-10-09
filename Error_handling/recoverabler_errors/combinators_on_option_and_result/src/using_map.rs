fn get_nth(items: &Vec<usize>, nth: usize) -> Option<usize> {
    if nth < items.len() {
        Some(items[nth])
    } else {
        None
    }
}

pub fn using_map() {
    let items = vec![7, 6, 5, 4, 3, 10, 3, 2, 4];
    println!("{}", items.len());
    let doubled = get_nth(&items, 9).map(|val| val * 2);
    println!("{:?}", doubled);
    println!("{}", items[4]);
    println!("{:?}", doubled);
}
