fn main() {
    let rect1 = build_rectangle(30, 50);
    let rect2 = build_rectangle(40, 40);
    let rect3 = build_rectangle(20, 20);

    rect1.print_area();
    rect2.print_area();
    rect3.print_area();

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
        self.area() > rect.area()
    }
}

fn build_rectangle(width: u32, height: u32) -> Rectangle {
    Rectangle { width, height }
}
