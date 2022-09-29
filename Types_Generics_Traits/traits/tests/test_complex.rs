use traits::complex::*;

#[test]
fn complex_basics() {
    let first = Complex::new(3, 5);
    let second: Complex<i32> = Complex::default();
    assert_eq!(first.re, 3);
    assert_eq!(first.img, 5);
    assert!(second.re == second.img);
}

#[test]
fn complex_addition() {
    let a = Complex::new(1, -2);
    let b = Complex::default();
    let res = a + b;
    assert_eq!(res, a);
}
