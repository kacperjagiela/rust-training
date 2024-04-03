use add_one;

use add_two;

fn main() {
    let num = 10;

    println!("Hello, world! {num} plus one is {}", add_one::add_one(num));

    let num2 = 15;

    println!(
        "Hello, world! {num2} plus one is {}",
        add_two::add_two(num2)
    );
}
