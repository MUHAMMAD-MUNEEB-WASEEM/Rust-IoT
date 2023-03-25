
fn main(){
    let some_number = Some(5);              //Some take value of data type u32 or i32
    let some_string = Some(String::from("Hello, how are u?")); //some take of datatype String 
    println!("{:#?}", some_number);
    println!("{:#?}", some_string);

    //enum option is already predefined in Rust, so we dont need to made it every time, this is just for
    //learning purpose, we can directly use Some and None, but with None we have to attach some kind of
    //data type, because Rust is static programming language, its ensure that every variable has some
    //data type.

    let none_value: Option <i32> = None; //as we have mentioned that every variable takes value as in Some takes u32 and String so for None, we have defined i32
    println!("{:#?}", none_value);
}