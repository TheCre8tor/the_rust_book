use rand::Rng;
use std::{cmp::Ordering, io};

/* NOTE: You won’t just know which traits to use and which
methods and functions to call from a crate, so each crate
has documentation with instructions for using it. Another
neat feature of Cargo is that running the cargo doc --open
command will build documentation provided by all of your
dependencies locally and open it in your browser. If you’re
interested in other functionality in the rand crate, for
example, run cargo doc --open and click rand in the sidebar
on the left. */

pub fn run() {
    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Please input your guess.");

        /* The :: syntax in the ::new line indicates that
        new is an associated function of the String type */
        let mut guess = String::new();

        /* read_line takes whatever the user types into
        standard input and append that into a string.

        The & indicates that this argument is a reference,
        which gives you a way to let multiple parts of your
        code access one piece of data without needing to
        copy that data into memory multiple times.

        references are immutable by default. Hence, you
        need to write &mut guess rather than &guess to make
        it mutable.*/
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /* helpfully Rust allows us to shadow the previous
        value of guess with a new one.

        ? Shadowing lets us reuse the guess variable
        ? name rather than forcing us to create two unique
        ? variables

        The parse method on strings parses a string into
        some kind of number.

        ! The parse method will only work on characters that
        ! can logically be converted into numbers and so can
        ! easily cause errors */
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🤠 We expect number only!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        /* A match expression is made up of arms. An arm
        consists of a pattern to match against, and the
        code that should be run if the value given to
        match fits that arm’s pattern. */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("🥳 You won!");
                break;
            }
        }
    }
}
