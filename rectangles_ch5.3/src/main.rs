#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }    

    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.height > rect.height && self.width > rect.width {
            return true
        }

        false
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 50
    };

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

    // println!("{:#?}", rect)
    println!("This area of the rectangle is {} square pixels.", rect.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
