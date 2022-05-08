#[path = "./the_string_type.rs"]
mod the_string_type;

pub fn _owner() {
    println!("This is ownership");

    /* SECTION  1. What is ownership:
    Ownership is a set of rules that governs how a Rust program manages memory.

    ? The Stack ->
      The stack stores values in the order it gets them and removes the values
      in the opposite order. This is referred to as last in, first out.

      * Adding data is called pushing onto the stack, and removing data is
      * called popping off the stack.

      All data stored on the stack must have a known, fixed size. Data with an
      unknown size at compile time or a size that might change must be stored
      on the heap instead.

    ? The Heap ->
      The heap is less organized: when you put data on the heap, you request a
      certain amount of space. The memory allocator finds an empty spot in the
      heap that is big enough, marks it as being in use, and returns a pointer,
      which is the address of that location.

      ? Because the pointer to the heap is a known, fixed size, you can store
      ? the pointer on the stack, but when you want the actual data, you must
      ? follow the pointer.

      * This process is called allocating on the heap and is sometimes
      * abbreviated as just allocating.

      * Pushing values onto the stack is not considered allocating.

      Pushing to the stack is faster than allocating on the heap because
      the allocator never has to search for a place to store new data;
      that location is always at the top of the stack.

      * Accessing data in the heap is slower than accessing data on the stack
      * because you have to follow a pointer to get there.

      ? When your code calls a function, the values passed into the function
      ? (including, potentially, pointers to data on the heap) and the
      ? function’s local variables get pushed onto the stack. When the function
      ? is over, those values get popped off the stack.

      ! The main purpose of ownership is to manage heap data.

    */

    /* SECTION  2. Ownership Rules ->
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    // 1. Variable Scope ->

    {
        // "s" is not valid here, it's not yet declared
        let _s = "hello"; // "s" is valid from this point foward

        // do stuff with s
    } // this scope is now over, and "s" is no longer valid

    /* The variable is valid from the point at which it’s declared until
    the end of the current scope.

    In other words, there are two important points in time here:

    1. When s comes into scope, it is valid.
    2. It remains valid until it goes out of scope.

    At this point, the relationship between scopes and when variables
    are valid is similar to that in other programming languages.*/

    the_string_type::_run();
}
