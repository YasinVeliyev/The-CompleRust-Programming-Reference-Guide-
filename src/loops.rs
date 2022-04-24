#[allow(dead_code, unused_labels, unused_attributes)]
fn silly_sub(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increment: loop {
        if result == a {
            let mut dec = b;
            'decrement: loop {
                if dec == 0 {
                    break 'increment;
                } else {
                    result -= 1;
                    dec -= 1
                }
            }
        } else {
            result += 1;
        }
    }
    result
}
#[allow(dead_code, unused_labels, unused_attributes)]
fn forloop() {
    for i in 0..=10 {
        print!("{i} ")
    }
}
#[allow(dead_code, unused_labels, unused_attributes)]
pub fn call_loop() {
    let (a, b) = (10, 4);
    let result = silly_sub(a, b);
    println!("{} minus {} is {}", a, b, result);
    forloop();
}
