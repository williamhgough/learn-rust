use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut map = HashMap::new();
    map.insert(String::from("Favourite color"), String::from("Blue"));

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    map.entry(String::from("Favourite color"))
        .or_insert(String::from("Green"));

    println!("{:?}", map);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
