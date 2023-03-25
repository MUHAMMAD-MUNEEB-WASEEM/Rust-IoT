#[derive(Debug)]

enum Message {
    Quit,
    Write(String),
}

fn call (x: Message){           //function that takes parameter of type enum 
    println!("{:#?}", x);       // print that specific parameter
}

fn main() {
    let msg1 = Message::Quit;   //instances of enum
    let msg2 = Message::Write(String::from("Hello, How are u ?"));

    call(msg1);                 //passing instance as argument to function to print their value
    call(msg2);
}


