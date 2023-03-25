use std::collections::HashMap;


fn main() {

    //creating hashmap using collect method
    //collect method is used to create hashmap when both keys and values are in vectors
    //collect combine key with correspong value
    
    let team = vec![String::from("Blue"), String::from("Yellow")];          //keys
    let  initial_score = vec![10, 20, 30];                                  //values
   
    //you can notice that here values are 3 where as keys are 2, so third value will going to be ignore
    //as we have only 2 keys

    let map:HashMap<_, _> = team.iter().zip(initial_score.iter()).collect();
                         // all keys    all values                combining
    println!("{:?}", map);

    //accessing key value pair using .get

    let key = String::from("Yellow");
    let yellow_key = map.get(&key);
    println!("{:?}", yellow_key);

    //accessing key value pair using for loop

    for (k, v) in &map{
        println!("key is {} and value is {}", k, v);
    }

}