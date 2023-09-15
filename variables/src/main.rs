fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // explicit type declaration must exist for parse() to work

    println!("guess: {}", guess);
}
