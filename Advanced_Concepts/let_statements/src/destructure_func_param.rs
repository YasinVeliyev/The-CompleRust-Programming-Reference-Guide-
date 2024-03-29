struct Container {
    items_count: u32,
}

fn increment_item(Container { mut items_count }: &mut Container) {
    items_count += 1;
}

fn calculate_cost(Container { items_count }: &Container) -> u32 {
    let rate = 67;
    rate * items_count
}

pub fn destructure_func_param() {
    let mut container = Container { items_count: 10 };
    increment_item(&mut container);
    let total_cost = calculate_cost(&container);
    println!("Total Cost: {}", total_cost);
}
