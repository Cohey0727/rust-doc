fn main() {
    main_5_1();
    main_5_2();
    main_5_3();
}

fn main_5_1() {
    println!("5.1. Defining and Instantiating Structs");
    let user_1 = create_user();
    println!(
        "email: {}, username: {}, sign_in_count: {}, active: {}",
        user_1.email, user_1.username, user_1.sign_in_count, user_1.active
    );
    let user_2 = User {
        username: String::from("Hello"),
        ..user_1
    };
    println!("{}", user_1.username);
    // println!("{}", user_1.email); user_1はemailは所有権を失っているのでもはやアクセスできない。
    println!("{}", user_2.email); // user_2では可能。
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user() -> User {
    User {
        email: String::from("hello_world@example.com"),
        username: String::from("hello_world"),
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main_5_2() {
    println!("5.2. An Example Program Using Structs");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main_5_3() {
    println!("5.3. Method Syntax");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };
    println!("rect1 is {}", rect1.area());
    println!("rect1 holding rect2 is {}", rect1.can_hold(&rect2));
}
