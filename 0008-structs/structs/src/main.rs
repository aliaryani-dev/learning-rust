// a simple struct demonstration for storing account info
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        email: String::from("lenesis@proton.me"),
        username: String::from("aliaryani-dev"),
        active: true,
        sign_in_count: 1,
    };
}
