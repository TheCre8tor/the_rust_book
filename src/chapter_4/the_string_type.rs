

pub fn _run() {
    // This is on Stack, and cannot be mutated ->
    let mut _unmute = "This &str cannot be mutated.";

    // This is on Heap, and can be mutated ->
    let mut stra = String::from("Hello");

    stra.push_str(", world."); // push_str() appends a literal to a String

    println!("{}", stra); // This will print `hello, world!`

    /* SECTION  1. Memory Allocation ->
    In the case of a string literal, we know the contents at compile time, so the
    text is hardcoded directly into the final executable. This is why string
    literals are fast and efficient. But these properties only come from the string
    literal’s immutability.

    With the String type, in order to support a mutable, growable piece of text, we
    need to allocate an amount of memory on the heap, unknown at compile time, to
    hold the contents.

    This Means:
        1. The memory must be requested from the memory allocator at runtime.
        2. We need a way of returning this memory to the allocator when we’re done
        with our String

    * That first part is done by us: when we call String::from, its implementation
    * requests the memory it needs. This is pretty much universal in programming
    * languages.

    * The second part is different. Rust takes a different path: the memory is
    * automatically returned once the variable that owns it goes out of scope.
     */


}
