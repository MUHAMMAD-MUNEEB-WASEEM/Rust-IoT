//1. First we will define struct
//2. we will define methods using Impl and custom datatype(Struct)
//3.then we will create object of struct in main and call methods of struct using that object in main

//1
struct Rectangle{
    height: u32,
    width: u32,
}

//2
impl Rectangle {
    fn area(&self) ->u32//first method, returns area of type u32
    {
        self.height*self.width 
    }
}

//3
fn main() {

    let rec_1 = Rectangle{height:5, width:4};//creating obj or instance
    let result = rec_1.area();//calling method using object
    println!("The area of rectangle is {}", result);

}
