#[derive(Debug)]

enum Message{
    Quit,
    Write(String),
    Move{x: i32, y: i32},
    ChangeColor(u32, u32, u32),

}
impl Message{                       //method for enum
    fn call(&self){                 //this method will be used with instance of enum, and prints 
                                    //particular enum instance
        println!("{:#?}", self)
    }

}

fn main() {
    let msg1 = Message::Quit;       //instance of enum
    msg1.call();                    //printing instance enum

}
