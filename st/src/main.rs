fn main() {
    struct User {
        email: String,
        username: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("test123@example.com"),
        username: String::from("test123"),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("test234");

    println!("{}", user1.username);
}
