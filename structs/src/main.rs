struct User {
    name: String,
    email: String,
    login_count: u64,
    active: bool
}

fn main() {
    let user = User {
        email: String::from("example@abc.com"),
        name: String::from("example"),
        login_count: 0,
        active: true
    };
    
    
}
