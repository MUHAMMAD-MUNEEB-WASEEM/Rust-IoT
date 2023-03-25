use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Blue"), 20);
    map.insert(String::from("yellow"), 50);

    println!("{:?}", map);
    map.entry(String::from("orange")).or_insert(100);//check orange has value or not, if it has then it will return it, otherwise assign new value 
    println!("{:?}", map);

}