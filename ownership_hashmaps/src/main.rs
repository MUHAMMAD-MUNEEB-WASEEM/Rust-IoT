use std::collections::HashMap;

fn main() {
    
    let key = String::from("favourite color");
    let value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(key, value);

    println!("{:?}", map);

    //as key and value have string type which is stored in heap, so after passing them to insert method
    //ownership is transferred to insert, now if i will going to print key and value they will give an 
    //ownership error, however if key and value have static data like int, float then we know that 
    //their is no concept of ownership, so we can still print key and value
}   
