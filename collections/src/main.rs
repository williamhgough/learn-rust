fn main() {
    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    let fourth: Option<&i32> = v.get(3);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.11),
        SpreadsheetCell::Text(String::from("blue")),
    ];
} // <- vec & v go out of scope here

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
