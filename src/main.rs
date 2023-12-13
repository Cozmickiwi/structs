fn main() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    println!("{}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
    let user2 = User {
        email: String::from("another@example123.com"),
        ..user1
    };
    println!("{}", user2.username);
    
    let black = Color(0, 0, 0);
    let origin = Point(82, 19, 6);
    println!("Color is: ({}, {}, {}) and origin is: ({}, {}, {}).", black.0, black.1, black.2, origin.0, origin.1, origin.2);
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}