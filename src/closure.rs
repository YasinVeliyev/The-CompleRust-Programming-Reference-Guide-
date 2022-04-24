pub fn call_closure() {
    let double = |x| x * 2;
    let value = 5;
    let twice = double(value);
    println!("{} doubled is {}", value, twice);
    let big_closure = |b: i32, c: i32| {
        let z = b + c;
        z * twice
    };

    let some_number = big_closure(1, 2);
    println!("Result from closure: {}", some_number);
}
