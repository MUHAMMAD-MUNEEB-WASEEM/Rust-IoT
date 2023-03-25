/// FIRST METHOD OF DATA INPUT INCASE OF ENUM USING STRUCT


#[derive(Debug)]

enum IPaddresskind{                             //defining enum for categorical or classified data
    V4,                             
    V6
}

#[derive(Debug)]
struct IPaddress{                               //defining struct to deal with more data, 
                                                //in kind key, we used IPaddresskind(enum data type)
    kind: IPaddresskind,
    address : String,
}


fn main() {
    let ipaddress1 = IPaddress{                 //creating instance of struct which is using enum data type
        kind : IPaddresskind::V4,
        address : String::from("127.0.0.1"),
    };

    println!("{:#?}", ipaddress1);
}
