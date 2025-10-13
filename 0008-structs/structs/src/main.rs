// a simple struct demonstration for storing account info
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// tuple structs
struct Color(i32, i32, i32);

fn build_user (email:String, username:String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(String::from("lenesis@proton.me"), String::from("aliaryani-dev"));
    
    user1.username = String::from("aliaryani.dev");
    println!("user1's email: {}",user1.email);
    if user1.active {
        println!("sign-in count: {}",user1.sign_in_count);
    }

    // struct update syntax
    let user2 = User {
        email: String::from("example@mail.com"),
        ..user1
    };
    println!("user2: username: {}, active state: {}, email: {}",user2.username,user2.active,user2.email);

    // declare tuple struct instance
    let black = Color(0,0,0);
    println!("black: ({},{},{})",black.0,black.1,black.2);
}
