pub fn control() {
    // 1. if Expressions

    let number: u8 = 14;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition is false");
    }

    // 2. Handing Multiple Conditions with else if

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 3. Using if in a let statement
    let condition = true;
    let assign_number: u8 = if condition {10} else {12};

    println!("The value of assign_number is: {}", assign_number);
}