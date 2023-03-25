#[derive(Debug)]

enum Spreadsheet{
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let v1 = vec![Spreadsheet::Int(12), Spreadsheet::Float(2.4), Spreadsheet::Text(String::from("Hello"))];
    println!("{:#?}", v1);

}
