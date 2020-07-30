fn main() {
    println!("1. 矩形面积是 {} 平米.", area1(30, 50));

    println!("2. 矩形2面积是 {} 平米.", area2((30, 50)));

    println!(
        "3. 矩形2面积是 {} 平米.",
        area3(&Rectangle {
            width: 30,
            height: 50,
        })
    );

    println!(
        "矩形 {:?}",
        Rectangle {
            width: 30,
            height: 50,
        }
    );

    println!(
        "矩形 pretty {:#?}",
        Rectangle {
            width: 30,
            height: 50,
        }
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
