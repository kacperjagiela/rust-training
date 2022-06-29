fn main() {
    println!("Hello, world!");

    second_function();
    another_function(125);
    print_labeled_measurement(2137, 'e');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y: {}", y);

    let y = five();
    println!("The value of y: {}", y);

    let y = plus_one(y);
    println!("The value of y: {}", y);
}

fn second_function() {
    println!("Second function");
}

fn another_function(x: i32) {
    println!("The value of x: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
