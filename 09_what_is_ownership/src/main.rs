fn main() {
    {
        let s = "hello";
        println!("{s}");
    }

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    // Clone

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");

    takes_ownership(s);
    // s's value moves into the function and is no longer valid here

    let x = 5;

    makes_copy(x);
    // x would move into function but i32 is Copy so it's still valid here

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("s1: {s1}");
    println!("s3: {s3}");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
