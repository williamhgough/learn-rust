fn main() {
    let s = String::from("hello"); // s comes into scope.

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here.

    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the3 function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward.

    let mut n = String::from("hello");
    change(&mut n);
    println!("{}", n);

    let mut s = String::from("Hello!");
    let _r1 = &mut s;

    let _reference_to_nothing = no_dangle();
} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world!")
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
