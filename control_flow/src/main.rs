fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("LIFTOFF!!!");
}
