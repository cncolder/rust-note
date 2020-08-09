// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// 模块内部默认都是私有的.

mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
