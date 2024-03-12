use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Looping

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // Updating hashmap

    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Yellow"), 25);

    println!("{:?}", scores);

    // Only if key isn't present

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating based on old value

    let text = "hello world woderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{:?}", map);
}
