#[derive(Debug)]

//define tuple struct
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    //creating instance of tuple struct
    let black = Color(6, 9, 0);
    print_tuple(black);//passing color struct instance to function for printing
    let axis = Point(0, 1, 0);//creating instance of point tuple
    println!("{:#?}", axis);//printing instance

}
fn print_tuple(x:Color)//making function that take input of color data type and print it
{
    println!("{:#?}", x);
}