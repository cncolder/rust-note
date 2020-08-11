fn main() {
    generic_trait();

    operator_overloading();

    operator_overloading_with_difference();

    same_method_name();

    super_trait();

    newtype();
}

pub trait Iterator {
    // 类型占位符, 具体类型由实现方决定.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn generic_trait() {
    /// 为什么不使用泛型
    /// 如果使用泛型, 特征是可以实现多次的, 每次实现指定不同的类型, 最终调用 `next` 的用户也必须指定泛型...
    trait Iterator2<T> {
        fn next(&mut self) -> Option<T>;
    }
    struct Counter2 {}
    impl Iterator2<i32> for Counter2 {
        fn next(&mut self) -> Option<i32> {
            Some(1)
        }
    }
    impl Iterator2<u64> for Counter2 {
        fn next(&mut self) -> Option<u64> {
            Some(1)
        }
    }

    let mut c = Counter2 {};
    // println!("{:?}", c.next());
}

fn operator_overloading() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    println!("两点相加 {:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
}

fn operator_overloading_with_difference() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Millimeters(u32);
    struct Meters(u32);

    // Add 特征有个泛型参数, 默认是 `Self`, 代表与自己相加的类型和自己相同, 此处指定一个不同类型.
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        // 毫米加米, 返回毫米.
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    println!("长度相加 {:?}", Millimeters(100) + Meters(1));
}

fn same_method_name() {
    /// 飞行器
    trait Pilot {
        fn fly(&self);
    }

    /// 巫师
    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("科技飞行");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("法术飞行");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("梦里飞");
        }
    }

    let person = Human;

    // Pilot 方法
    Pilot::fly(&person);
    // Wizard 方法
    Wizard::fly(&person);
    // Human 方法
    Human::fly(&person);
    // 实例自身方法.
    person.fly();

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("小斑点")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("小奶狗")
        }
    }

    // 关联函数(静态方法)会调用自身的方法, 不受其他 trait 影响 .
    println!("小狗一般叫 {}", Dog::baby_name());

    // 如果要指定调用 Animal 特征的方法, 需要用到 fully qualified syntax.
    // 注意尖角号, 不是括号.
    // 只有多个实现使用了相同的方法名时, 才会用到这个详尽语法.
    println!("小狗一般叫 {}", <Dog as Animal>::baby_name());
}

fn super_trait() {
    use std::fmt;

    /// 打印内容到星号边框
    ///
    /// # Examples
    ///
    /// ```
    /// **********
    /// *        *
    /// * (1, 3) *
    /// *        *
    /// **********
    /// ```
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            // 指定 OutlinePrint 特征需要 Display 特征, 所以我们可以直接调用 `self.to_string()`.
            let output = self.to_string();
            let len = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 在此之前要先实现 Display
    impl OutlinePrint for Point {}

    let point = Point { x: 1, y: 2 };

    point.outline_print();
}

fn newtype() {
    use std::fmt;
    use std::ops;

    /// 默认情况下, 当我们为类型实现 trait 时, 二者必有其一是在本地 crate 中的.
    /// 如果无法满足这个条件, 可以使用最轻的 tuple 包装目标类型, 然后为这个类型实现 trait.
    /// Wrapper 是个新类型, 它没有 Vec 的方法.
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    /// 要想让 Wrapper 行为和 Vec<String> 相同, 最简单的办法是实现 Deref 特征.
    /// 这样就可以把 Wrapper 当 Vec<String> 来用了.
    impl ops::Deref for Wrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Vec<String> {
            &self.0
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);

    println!("w = {}", w);
}
