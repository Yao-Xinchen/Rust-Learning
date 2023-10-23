struct User {
    username: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32); // tuple struct
struct Point(i32, i32, i32); // tuple struct

fn main() {
    let mut usr1 = User {
        username: String::from("yao"),
        sign_in_count: 1,
        active: true,
    };

    usr1.username = String::from("xinchen");

    let usr2 = User {
        active: false,
        ..usr1
    };

    // println!("usr1: {}", usr1.username);
    println!("usr2: {}", usr2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
