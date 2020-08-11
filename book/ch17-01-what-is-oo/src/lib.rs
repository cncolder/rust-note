//! OO 面向对象
//!
//! 对象的概念来自于 1960 年诞生的 Simula 语言.
//!
//! Rust 使用 `struct` 保存数据, 使用 `impl` 实现方法, 默认 private 隐藏实现细节. 但不支持继承. Rust 使用 Trait 确保参数符合某种特征, 而不是传统的要求参数必须是某个类的子类这种多态方式.
//!
//! 继承在如今的语言设计领域不受待见, 因为子类总是别无选择的带上父类所有东西, 并且只能有一个父类.

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
