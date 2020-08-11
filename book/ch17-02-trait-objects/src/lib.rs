//! Trait 特征
//!
//! Trait 更像其他语言里的 Object, 联合数据和行为, 区别是我们不能向 trait 添加数据. Trait 的目的是公共抽象行为.
//!
//! 假如想实现一个 `gui` 库, 每个组件都有一个 `draw` 方法, 一般来说(例如 React)需要定义一个 `Component` 父类, 所有组件都继承这个父类, 重写 `draw` 方法.
//!
//! 以下示例中的 Trait 必须是对象安全的.
//! 所有方法必须符合两条规则:
//! - 不能返回 `Self` 类型.
//! - 不能带有泛型参数.

/// 绘图
pub trait Draw {
    fn draw(&self);
}

/// 屏幕
///
/// 所有组件都具备绘图特征
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/// 按钮
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

// 使用泛型可以在编译时验证类型, 但是要求集合元素必须同质.
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
