fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading elements

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // Out of range example

    let v = vec![1, 2, 3, 4];

    //let does_not_exist = &v[100];
    let does_not_exits = v.get(100);

    let mut v = vec![1, 2, 3];

    let first = &v[0];

    //v.push(6);

    println!("The first element is: {first}");

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // using enums with vectors

    enum SpeadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpeadsheetCell::Int(3),
        SpeadsheetCell::Float(64.32),
        SpeadsheetCell::Text(String::from("blue")),
    ];

    {
        let v = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed here
}
