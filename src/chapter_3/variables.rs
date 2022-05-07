pub fn _run() {
    // 1. Variables and Immutability -->
    let mut value = 6;
    println!("The value of x is: {}", value);

    value = 9;
    println!("The value of x is: {}", value);

    // 2. Constants -->
    /* Constants are values that are bound to a
    name and are not allowed to change */

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Time of the day: {} ", THREE_HOURS_IN_SECONDS);

    // 3. Shadowing -->
    /* Rustaceans say that the first variable is shadowed
    by the second, which means that the second variable’s
    value is what the program sees when the variable is
    used. */

    let shadow = 8;
    let shadow = shadow + 1;

    {
        let shadow = shadow * 2;
        println!("The value of x in the inner scope is: {}", shadow);
    }

    println!("The value of x is: {}", shadow);
}
