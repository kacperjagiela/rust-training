fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("{word}");

    // starting slice
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    // ending slice

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // entire string slice

    let slice = &s[0..len];
    let slice = &s[..];

    // first_word_2 implementation

    let s = String::from("hello world");

    let word = first_word_2(&s);

    println!("{word}");

    let word = first_word_2(&s[0..6]);
    println!("{word}");

    let s = "hello worlds";

    let word = first_word_2(&s[0..6]);

    println!("{word}");

    // other slices

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
