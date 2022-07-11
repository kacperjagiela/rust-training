struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "User {} signed in {} times!",
        &user1.username, &user1.sign_in_count
    );

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 1,
    }
}
