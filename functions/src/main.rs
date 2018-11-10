fn main() {
    let x = plus_one(4);

    println!("The value of x is: {}", x);

    if x < 4 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("{}", elem);
    }

    println!("reversed:");
    for number in (1..10).rev() {
        println!("{}", number);
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n)
    }
    match n {
        0 => panic!("zero is incorrect"),
        1 | 2 => 1,
        3 => 3,

        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
