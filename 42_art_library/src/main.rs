use art_library::{mix, PrimaryColor};

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, &yellow);
}
