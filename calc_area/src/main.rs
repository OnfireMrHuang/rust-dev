
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数，通常作为构造函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    // 方法
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}

fn main() {

    let rect1 = Rectangle{ width: 30, height: 50};

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

