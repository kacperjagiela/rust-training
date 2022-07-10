fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");

    let mut str = String::from("hello");

    {
        let r1 = &mut str;

        println!("{}", r1);
    }
    let r2 = &mut str;

    println!("{}", r2);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // scope ends for r1 and r2

    let r3 = &mut s;
    println!("{}", r3);

    // dangling references
    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}
