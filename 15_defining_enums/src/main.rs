fn main() {
    enum IpAddressKind {
        V4,
        V6,
    }

    struct IpAddress {
        kind: IpAddressKind,
        address: String,
    }

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };

    // Putting data into enums

    enum IpAddressData {
        V4(String),
        V6(String),
    }

    let home = IpAddressData::V4(String::from("127.0.0.1"));

    let loopback = IpAddressData::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {}
    }

    let m = Message::Write(String::from("Hello"));

    m.call();

    // Option type

    let some_number = Some(5);

    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
