fn main() {
    let u1 = build_user(
        String::from("williamhgough"),
        String::from("whg@devtheweb.io"),
    );

    let u2 = User {
        email: String::from("akk@devtheweb.io"),
        username: String::from("avekies"),
        ..u1 // Struct Update syntax
    };

    println!("{:#?}", u1);
    println!("{:#?}", u2);

    let black = Color(0, 0, 0);
    println!("{:#?}", black);
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        email, // Field init shorthand syntax
        username,
        active: true,
        sign_in_count: 1,
    }
}
