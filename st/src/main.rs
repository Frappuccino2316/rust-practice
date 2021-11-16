fn main() {
    let user1 = build_user(String::from("user1@example.com"), String::from("user1"));

    println!("{}", user1.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}
