#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is a method -> Instantiated Method
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, data: &Rectangle) -> bool {
        self.width > data.width && self.height > data.height
    }

    // This is similar to a factory/abstract function ->
    // Associated Function
    fn from(height: u32, width: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
}

pub fn run() {
    let rectangle = Rectangle {
        width: 20,
        height: 30
    };

    let associate = Rectangle::from(50, 30);

    println!("The area of the rectangle is {} square pixels.", rectangle.area());
    println!("This is the result of an associated function: {:?}", associate.area());
    println!("Can we create a rectangle? {}", associate.can_hold(&rectangle));
}