#[derive(Debug)]
struct Rectangle
{
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {//new instance of struct rectangle
            width: size,
            height: size,
        }
    }
}

fn main()
{
    let result = Rectangle::square(8);//struct name followed by namespace and associated function name
    println!("{:#?}", result);
}
