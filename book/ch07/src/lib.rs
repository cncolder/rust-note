pub use crate::front_of_house::hosting;
use rand::Rng;
use std::{fmt::Result, io::Result as IoResult};

mod front_of_house;

pub fn eat_at_restaurant() {
    // 绝对路径
    hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house;

fn main() {}
