#[path = "./the_slice_type.rs"]
mod the_slice;

pub fn run() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    /* NOTE: you can have only one mutable reference to a
    particular piece of data at a time in a scope. */

    let mut muta = String::new();

    {
        // New Scope ->
        let _r3 = &mut muta.push_str("scoped ");
    }

    let _r1 = &mut muta.push_str("global");
    // let r2 = &mut muta; // -> Borrowing mutable reference twice will cause an error

    let h = &muta;

    println!("Result: {}", muta);
    println!("Let's be straight with {}", muta);
    println!("{}", h);

    the_slice::run();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
