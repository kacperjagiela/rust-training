fn main() {
    // match

    let x: Option<i32> = Some(13);

    match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    // if let

    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourite color, {color}");
    } else if is_tuesday {
        println!("Thursday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as color");
        } else {
            println!("Using orange as color");
        }
    } else {
        println!("Using blue as color")
    }

    // while let

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at the index {}", value, index);
    }

    // let statements

    let (x, y, z) = (1, 2, 3);

    // function parameters

    fn foo(x: i32) {
        println!("{}", x);
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
