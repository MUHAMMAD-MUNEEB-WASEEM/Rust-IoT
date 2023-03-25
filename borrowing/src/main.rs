fn main() {
    let s = String::from("Unique");
    let result = length(&s);
    println!("The length of word {} is {}", s, result);
}
fn length(x: &String) -> usize
{
    x.len()
}
