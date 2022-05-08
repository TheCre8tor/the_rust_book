pub fn run() {
    let word = String::from("How are you!");

    let result = first_world(&word);

    println!("First word is: {}", result);

    // SECTION  2. String Slices
    /* A string slice is a reference to part of a String, 
       and it looks like this: */

    let string = String::from("Hello world! ");

    let hello = &string[0..5];
    let world = &string[6..12];

    println!("First Slice: {}, Second Slice:{}", hello, world);

    // SECTION  3. String Literals Are Slices
}

fn first_world(stra: &String) -> &str {
    let word = stra.as_bytes();

    for (idx, &item) in word.iter().enumerate() {
        if item == b' ' {
            return &stra[0..idx];
        }
    }

    &stra[..]
}