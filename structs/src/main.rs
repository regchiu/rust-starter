#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method Syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Tuples Struct
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit-Like Structs
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("user1"),
        active: true,
        sign_in_count: 1,
    };
    dbg!(&user1);

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };
    dbg!(&user2);

    let black = Color(0, 0, 0);
    dbg!(&black);
    let origin = Point(0, 0, 0);
    dbg!(&origin);

    let subject = AlwaysEqual;

    dbg!(&subject);

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    dbg!(&sq);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
