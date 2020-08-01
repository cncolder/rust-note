#[allow(unused)]

fn main() {
    let p1 = Pair { x: 5, y: 10.4 };
    let p2 = Pair { x: "hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("最大的数字是 {}", largest(&vec![34, 50, 25, 100, 65]));
    println!("最大的字符是 {}", largest(&vec!['y', 'm', 'a', 'q']));
}

use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    &largest
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Pair<T, U> {
    x: T,
    y: U,
}
impl<T, U> Pair<T, U> {
    fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair {
            x: self.x,
            y: other.y,
        }
    }
}
