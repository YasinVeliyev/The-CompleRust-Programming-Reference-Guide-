fn main() {
    let k = 40;
    let result_msg = match k {
        42 => "done",
        a if a % 2 == 0 => {
            println!("{}", a);
            "continue"
        }
        _ => panic!("Oh no !"),
    };
    println!("{}", result_msg);
}
