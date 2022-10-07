use std::cell::Cell;

struct Point {
    x: u8,
    y: u8,
    cached_sum: Cell<Option<u8>>,
}

impl Point {
    fn sum(&self) -> u8 {
        match self.cached_sum.get() {
            Some(sum) => {
                println!("Got from cache: {}", sum);
                sum
            }
            None => {
                let new_sum = self.x + self.y;
                self.cached_sum.set(Some(new_sum));
                new_sum
            }
        }
    }
}

pub fn uses_of_interior_mutability() {
    let p = Point {
        x: 8,
        y: 8,
        cached_sum: Cell::new(None),
    };

    println! {"Summed result {}",p.sum()};
    println! {"Summed result {}",p.sum()};
}
