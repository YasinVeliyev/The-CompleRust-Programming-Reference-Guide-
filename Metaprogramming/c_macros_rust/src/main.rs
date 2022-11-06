macro_rules! switch {
    ($a:expr,$b:expr)=>{
        let temp:i32;
        temp=$a;
        $b=$a;
        $a=temp;
    }
}

fn main() {
    let mut x=1;
    let mut y=2;
    let temp:i32;
    switch!(x,y);
    println!("x: {} y: {}",x,y);
}
