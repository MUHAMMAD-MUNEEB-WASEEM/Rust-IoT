use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 20);
    map.insert(String::from("yellow"), 50);

    println!("{:?}", map);

    map.insert(String::from("yellow"), 100);//overwriting yellow value
    println!("{:?}", map);

}