fn main() {
    let user1 = User {
        active: true;
        username: String::from("someuser123");
        email: String::from("someuser@example.com");
        sign_in_count: 1;
    };
}

struct User {
    active: bool;
    username: String,
    email: String;
    sign_in_count: u64;
}