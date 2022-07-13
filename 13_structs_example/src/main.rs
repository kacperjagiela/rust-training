#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("Area of rectangle is {}", calculate_area(width1, height1));

    let rect1 = (30, 50);

    println!("Area of rectangle is {}", calculate_area_tuple(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area of rectangle is {}", calculate_area_struct(&rect2));

    // print rectangle

    println!("rect2 is {:#?}", rect2);

    dbg!(&rect2);
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
