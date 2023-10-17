fn main() {
    let s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5

    // s.clear(); // cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if item == b' ' { // b' ' is a byte literal
            return &s[..i]; // return a slice of the string from 0 to i
        }
    }
    &s[..] // return the whole string
}