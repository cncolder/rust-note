use std::fmt::*;

mod pair;

pub trait Summary {
    fn summarize_author(&self) -> String;

    // trait 方法可以有默认实现, 默认实现可以调用其他方法, 包括没有默认实现的方法.
    fn summarize(&self) -> String {
        format!("查看详情 ({})...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 实现 Summary trait 为 Tweet
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // 覆盖默认实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 书里说这种写法是下面的语法糖?
// pub fn notify(item: &impl Summary) {
//     println!("重大新闻! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("重大新闻! {}", item.summarize());
}

// 1 和 2 都实现了 Summary.
// pub fn notify(item1: &impl Summary, item2: &impl Summary)

// 1 和 2 是实现了 Summary 的相同类型.
// pub fn notify<T: Summary>(item1: &T, item2: &T)

// pub fn notify(item: &(impl Summary + Display))
// pub fn notify<T: Summary + Display>(item: &T)

fn trait_bounds_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}

// 泛型很复杂时放在 where 里更清晰.
fn trait_bounds_where_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// 限制返回类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

