use std::collections::HashMap;

#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    let v = vec![1, 2, 3];
    println!("v: {:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("v: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {}", s);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores: {:?}", scores);
}
