pub fn run() {
    let word = String::from("How are you!");
    let result = first_world(&word);

    let str_slice = &word[0..result];

    println!("First word is: {}", str_slice);
}

fn first_world(stra: &String) -> usize {
    let bytes = stra.as_bytes();

    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return idx;
        }
    }

    stra.len()
}