// a simple struct demonstration for storing account info
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let mut user1 = User {
        email: String::from("lenesis@proton.me"),
        username: String::from("aliaryani-dev"),
        active: true,
        sign_in_count: 1,
    };
    
    user1.username = String::from("aliaryani.dev");
    println!("user1's email: {}",user1.email);
    if user1.active {
        println!("sign-in count: {}",user1.sign_in_count);
    }
}
