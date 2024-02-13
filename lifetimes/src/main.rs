// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: std::fmt::Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let str1 = longest(string1.as_str(), string2.as_str());
    let str2 = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Hello");
    println!("The longest string is {}", str1);
    println!("The longest string is {}", str2);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);
}

