fn main() {
    let rect1 = Rectangle::from(30, 50);
    let rect2 = Rectangle::from(40, 40);
    let rect3 = Rectangle::from(20, 20);
    let square = Rectangle::square(40);

    rect1.print_area();
    rect2.print_area();
    rect3.print_area();
    square.print_area();

    println!("Can rect1 hold rect2? {}", rect1.can_hold(rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn print_area(&self) {
        println!("{:#?}", self.area());
    }

    fn can_hold(&self, rect: Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn from(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
