#[derive(Debug)]

enum IPaddresskind{                     //here we havenot use struct for data
                                        //with V4, we can also pass string
                                        //with V6, we have passed tuple
                                        
    V4(String), 
    V6(u32, u32, u32),
}

fn main() {
    let ipaddress1 = IPaddresskind::V4(String::from("127.0.0.1"));
    let ipaddress2 = IPaddresskind::V6(127, 0, 1);

    println!("{:#?}", ipaddress1);
    }
