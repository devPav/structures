#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height 
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let sqr1 = Rectangle::square(50);
    println!("rect1 равен {:?}, его пложадь {}", rect1, rect1.area());
    println!("Может ли rect1 содержать в себе rect2? {}", rect1.can_hold(&rect2));
    println!("Может ли rect1 содержать в себе rect3? {}", rect1.can_hold(&rect3));
    println!("sqr1 равен {:?}", sqr1);
}
