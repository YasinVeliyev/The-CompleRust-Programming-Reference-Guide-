use std::convert::From;
use std::fmt::{Display, Formatter, Result};
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

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            re: value.0,
            img: value.1,
        }
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} + {}i", self.re, self.img)
    }
}
