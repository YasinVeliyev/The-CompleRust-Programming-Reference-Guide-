#[macro_export]
macro_rules! scanline {
    ($x:ident) => {{
        std::io::stdin().read_line(&mut $x).unwrap();
        $x.trim()
    }};
    () => {{
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim();
        s
    }};
}

pub fn first_macro() {
    // let mut input = String::new();
    // scanline!(input);
    // println!("I read {}", input);
    // println!("I read {}", scanline!());
    println!("{}", concat!("yasin", 5));
    
}
