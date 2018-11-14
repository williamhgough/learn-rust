use std::fs::File;
use std::io;
// use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3, 4];
    // v[99];

    let f = read_username_from_file();
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    println!("{}", f);

    // let e = File::open("hello.txt").unwrap();
    // let _e = File::open("hell.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
