use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}


fn main() -> Result<(), io::Error> {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = greeting_file_result.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error);
        }
    });

    let f = File::open("hello.txt")?;
    return Ok(());
}