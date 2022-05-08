#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn run() {
    let mut user = User {
        active: true,
        username: String::from("someuser1234"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user1 = build_user(String::from("someone@example.com"), String::from("someuser1234"));

    user.sign_in_count = 2;

    println!("User Data: {:#?}", user);
    println!("User1 Data: {:#?}", user1);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
