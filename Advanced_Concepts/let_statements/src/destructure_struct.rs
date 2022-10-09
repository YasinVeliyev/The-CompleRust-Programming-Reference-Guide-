#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    Pizza,
    Salad,
}

enum PaymentMode {
    Bitcoin,
    Credit,
}
struct Order {
    count: u8,
    food: Food,
    payment: PaymentMode,
}

pub fn destructure_struct() {
    let food_order = Order {
        count: 2,
        food: Food::Salad,
        payment: PaymentMode::Credit,
    };
    let Order { count, food, .. } = food_order;
    println!("{} {:?}", count, food);
}
