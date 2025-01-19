struct User {
    name: String,
    email: String,
    login_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        rect2.area() < self.area()
    }

    // static methods
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let user = User {
        email: String::from("example@abc.com"),
        name: String::from("example"),
        login_count: 0,
        active: true
    };

    let status = if user.active {
        "Enabled"
    } else {
        "Disabled"
    };

    println!("User {}'s email is {} and have have logged in for {} times. Their account is currently {}", user.name, user.email, user.login_count, status);

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!(
        "The area of the rectangle is {} square pixels.",
        &rect1.area()
    );

    println!(
        "Can rect1 hold rect2? {}", 
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect3? {}", 
        rect1.can_hold(&rect3)
    );

    println!(
        "A square from rectangle is {:#?}", 
        Rectangle::square(4)
    );

    println!(
        "The area of the rectangle is {:#?} square pixels.",
        rect1
    );
}
