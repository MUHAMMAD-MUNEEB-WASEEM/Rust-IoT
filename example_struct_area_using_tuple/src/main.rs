fn main() {
    let rec = (5, 4);
    println!("The area of rectangle is {}", area(rec));
}
fn area(dimensions: (u32, u32))->u32
{
    dimensions.0 * dimensions.1
}