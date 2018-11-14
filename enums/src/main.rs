fn main() {
    let _four = IpAddrKind::V4(127, 0, 0, 1);
    let _six = IpAddrKind::V6(String::from("::1"));

    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Penny);

    let six = plus_one(Some(5));
    let none = plus_one(None);

    let mut count = 0;
    if let Coin::Quarter(state) = Coin::Penny {
        println!("Quarter from state: {:?}", state);
    } else {
        count += 1;
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_type: IpAddrKind) {}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...etc
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => {
            println!("{:?}", i + 1);
            Some(i + 1)
        }
    }
}
