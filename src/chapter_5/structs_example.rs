#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run() {
    // SECTION  1. Refactoring with Structs: Adding More Meaning

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let calc = area(&rectangle);

    println!("The area of the rectangle is {} square pixels.", calc);
    dbg!(rectangle);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
