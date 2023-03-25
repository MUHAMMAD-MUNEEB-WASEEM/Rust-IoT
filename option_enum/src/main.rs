#[derive(Debug)]

enum Option <T> {           //option enum is used to tackle none data entry
    Some(T),                //It has two main categories; Some and None
    None,                   //None has no data entry, u can also call it as null value
}                           //Some is used for data entry where T represent we can store any type of data in it

fn main(){
    let some_number = Option::Some(5);          //passing int to some
    let some_string = Option::Some("Hello");    // passing string    

    println!("{:#?}", some_number);
    println!("{:#?}", some_string);
}
//enum option is already predefined in Rust, so we dont need to made it every time, this is just for
//learning purpose, we can directly use Some and None, but with None we have to attach some kind of
//data type, because Rust is static programming language, its ensure that every variable has some
//data type.