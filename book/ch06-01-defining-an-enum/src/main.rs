#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    V8 { l: u128, h: u128 },
}

impl IpAddr {
    fn call(&self) {

    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let atom = IpAddr::V8 { l: 1, h: 0 };

    println!(
        "home ip {:?}, loopback ip {:?}, atom ip {:?}.",
        home, loopback, atom
    );
}
