fn main() {
    let mut s1 = String::from("foo");
    s1.push_str(" bar");
    println!("{}", s1);

    let s2 = String::from("baz");
    println!("{} {}", s1, s2);
}
