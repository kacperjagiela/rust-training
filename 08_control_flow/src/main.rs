fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("numer was something other than zero");
    }

    let second_number = 6;

    if second_number % 4 == 0 {
        println!("number is divisible by 4")
    } else if second_number % 3 == 0 {
        println!("number is divisible by 3")
    } else if second_number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }

    let condition = true;
    let number_condition = if condition { 5 } else { 6 };

    println!("The value of number is: {number_condition}");

    let second_condition = false;
    let second_number_condition = if second_condition { 5 } else { 7 };

    println!("The value of number is: {second_number_condition}");

    // loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loops inside loops

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loops

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection with for

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value of element is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!");

    // fibonacci implementation

    let nth = 10;
    let mut first = 0;
    let mut second = 1;
    let mut counter = 1;

    let fibonacci_result = loop {
        counter += 1;
        if counter == nth {
            break first + second;
        }
        let temp = first;

        first = second;
        second += temp;
    };

    println!("Fibonaccis {nth} number is: {fibonacci_result}");
}
