pub fn _tuple() {
    let num_and_str = (40, "Have a good day");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("From tuple: Number: {}, String: {}", num, string);
    println!("{:?}", num_and_str);
}
