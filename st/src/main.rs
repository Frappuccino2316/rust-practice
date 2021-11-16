fn main() {
    struct User {
        email: String,
        username: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("test123@example.com"),
        username: String::from("test123"),
        sign_in_count: 1,
        active: true,
    };

    println!("{}", user1.username);
}
