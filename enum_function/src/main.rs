#[derive(Debug)]

enum IPaddresskind{
    V4, 
    V6,
}

fn call(x: IPaddresskind){      //making function that accepts parameter of enum datatype
    println!("{:#?}", x)        // print that parameter
}

fn main() {
   let ipaddress1 = IPaddresskind::V4; //making enum instance
   call(ipaddress1);             //passing it as argument to call function
}

