use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number");

    println!("{}", guess);

    let x = 2.0;
    let y: f32 = 3.0;

    println!("Values are x: {}, y: {}", x, y);

    // Numberic operations

    let sum = 5 + 10;

    let subtraction = 95.5 - 4.3;

    let multiplication = 4 * 30;

    let division1 = 56.7 / 32.2;
    let division2 = 2 / 3;

    let remainder = 43 % 5;

    let c = 'z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (i, j, k) = tup;

    println!("The value of j: {}", j);
    println!("The value of k: {}", tup.2);

    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
