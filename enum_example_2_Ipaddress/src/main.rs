#[derive(Debug)]

enum IPAddress{
    V4,
    V6
}

fn main() {
    let version_four = IPAddress::V4;
    let version_6 = IPAddress::V6;
    println!("{:#?}", version_6);
}

