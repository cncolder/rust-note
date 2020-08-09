use rand::Rng;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = rand::thread_rng().gen_range(1, 51);
    let simulated_random_number = rand::thread_rng().gen_range(1, 4);

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("正在计算...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("今天做 {} 个俯卧撑!", expensive_result.value(intensity));
        println!("之后再做 {} 个蹲起!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("休息一天, 记得坚持!");
        } else {
            println!("今天跑 {} 分钟!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T>
where
    // 把闭包保存在 struct 里, 对应的 trait 是 Fn. 一般情况下都是 Fn, 如果 Rust 发现不对, 会智能提示改用 FnOnce 或 FnMut.
    T: Fn(u32) -> u32,
{
    calculation: T,
    cache: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.cache.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn move_into_closure() {
        let x = vec![1, 2, 3];
        // 指定闭包中用到的变量会移动到内部.
        let equal_to_x = move |z| z == x;
        let y = vec![1, 2, 3];

        assert!(equal_to_x(y))
    }
}
