pub fn run() {
    // This is on Stack, and cannot be mutated ->
    let mut _unmute = "This &str cannot be mutated.";

    // This is on Heap, and can be mutated ->
    let mut stra = String::from("Hello");

    stra.push_str(", world."); // push_str() appends a literal to a String

    println!("{}", stra); // This will print `hello, world!`
}