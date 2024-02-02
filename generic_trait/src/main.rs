// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // fn distance_from_origin(&self) -> T {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let result = largest(&list);
    println!("The largest number is {}", result);

    let list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 1 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", integer);
    println!("{:?}", float);

    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: "".to_string(),
        content: "".to_string(),
    };
    println!("{}", news_article.summarize());
}
