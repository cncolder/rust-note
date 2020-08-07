#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    fn not_work() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // 自定义消息格式同 format!
        assert_eq!(4, add_two(2), "2 and 2 is {}", 4);
    }

    // 断言 panic!
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 直接用 Result 枚举当测试条件
    #[test]
    fn test_result() -> Result<(), String> {
        if (2 + 2 == 4) {
            Ok(())
        } else {
            Err(String::from("2加2不等于4"))
        }
    }

    #[test]
    #[ignore]
    fn not_run() {
        assert!(false);
    }
}
