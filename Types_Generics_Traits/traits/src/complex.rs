use std::ops::Add;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub img: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, img: T) -> Self {
        Self { re, img }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            img: self.img + rhs.img,
        }
    }
}
