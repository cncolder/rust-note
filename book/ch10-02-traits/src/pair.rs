use std::cmp::PartialOrd;
use std::fmt::Display;
use ToString;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if (self.x > self.y) {
            println!("最大数字是 x = {}", self.x);
        } else {
            println!("最大数字是 y = {}", self.y);
        }
    }
}
