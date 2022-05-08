pub fn _control() {
    // SECTION  1. if Expressions

    let number: u8 = 14;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition is false");
    }

    {
        // SECTION  2. Handing Multiple Conditions with else if
        print!("\n");

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    {
        // SECTION  3. Using if in a let statement
        print!("\n");

        let condition = true;
        let assign_number: u8 = if condition { 10 } else { 12 };

        println!("The value of assign_number is: {}", assign_number);
    }

    {
        // SECTION  4. Repitition with loops
        print!("\n");

        let mut count = 0;

        // The 'counting_up is a label
        'counting_up: loop {
            println!("count = {}", count);

            let mut remaining: u8 = 10;

            loop {
                println!("remaining = {}", remaining);

                if remaining == 9 {
                    break;
                }

                if count == 3 {
                    break 'counting_up;
                }

                remaining -= 1;
            }

            count += 1;
        }

        println!("End count = {}", count);
    }

    {
        // SECTION  5. Returning value from loops
        print!("\n");

        let mut counter: u8 = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");

        /* NOTE: One of the uses of a [loop] is to retry an
        operation you know might fail, such as checking
        whether a thread has completed its job. */
    }

    {
        // SECTION  6. Conditional Loops with [while]
        print!("\n");

        let mut while_loop: u8 = 3;

        while while_loop != 0 {
            println!("{}!", while_loop);

            while_loop -= 1;
        }

        println!("LIFTOFF!!!");

        /* NOTE: [while] construct eliminates a lot of nesting
        that would be necessary if you used [loop, if, else,
        and break], and it’s clearer. While a condition
        holds true, the code runs; otherwise, it exits the
        loop. */
    }

    {
        // SECTION  7. Looping Through a Collection with [for]
        print!("\n");

        /* This is an Array with fixed length */
        let boxes: [u8; 5] = [10, 20, 30, 40, 50];
        let mut idx = 0;

        while idx < 5 {
            println!("the value is: {}", boxes[idx]);

            idx += 1;
        }
        print!("\n");

        // Better Alternative
        for item in boxes {
            println!("the value is: {}", item);
        }
    }
}
