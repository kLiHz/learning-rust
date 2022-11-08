struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Rectangle) -> u32 {
        return self.height * self.width;
    }
    fn can_hold(self: &Rectangle, other: &Rectangle) -> bool {
        return self.width >= other.width && self.height >= other.height;
    }
    fn square(length: u32) -> Rectangle {
        Rectangle {
            width: length,
            height: length,
        }
    }
}

fn main() {
    let r1 = Rectangle { height: 20, width: 30 };
    let r2 = Rectangle::square(15);
    println!("R1's area is {}.", r1.area());
    println!("R2's area is {}.", r2.area());
    println!("Can R1 hold R1? {}", r1.can_hold(&r1));
    println!("Can R1 hold R2? {}", r1.can_hold(&r2));
}
