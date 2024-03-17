fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // in struct

    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 5.0, y: 10.0 };

    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let int_and_float = Point2 { x: 0, y: 1.2 };

    // in method definition

    struct Point3<T> {
        x: T,
        y: T,
    }

    impl<T> Point3<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point3<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point3 { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let point_f = Point3 { x: 0.4, y: 0.3 };

    println!("test: {}", point_f.distance_from_origin());

    struct Point4<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point4<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point4<X2, Y2>) -> Point4<X1, Y2> {
            Point4 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
