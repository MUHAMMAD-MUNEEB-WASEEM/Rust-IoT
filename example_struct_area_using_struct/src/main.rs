#[derive(Debug)]

struct Rectangle {
    height: u32,
    width: u32,
}


fn main() {
    let rec_1 = Rectangle {
        height: 5,
        width: 4
    };

    // println!("The area of rectangle is {}", area(rec_1));//rec_1 has now moved to area function
    // //main function does not has its ownership, so we cannot use here, if we want to use here
    // //then we have to use borrowing

    println!("The area of rectangle is {}", area(&rec_1));//as we rec_1 has been borrowed with area
    //function but main function has its ownership, so we can also use rec_1 in main function
    println!("{:#?}", rec_1);


}
fn area(dimension: &Rectangle) -> u32
{
    dimension.height * dimension.width
}