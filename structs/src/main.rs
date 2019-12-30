use structs::rect::Rectangle;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("some@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let test = build_user(String::from("test@example.com"), String::from("test"));
    println!("{}", test.email);

    let user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("user2"),
        ..user1
    };
    println!("{}", user2.active);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle1 is {} square pixels.",
        rect1.area()
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 //0번째 * 1번째
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
