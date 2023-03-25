fn main() {
    let mut v1 : Vec<i32> = Vec::new();     //empty vector

    //add values to vector

    v1.push(2);
    v1.push(4);
    v1.push(6);
    v1.push(8);

    println!("{:?}", v1);

    //remove values from vector

    v1.pop();

    println!("{:?}", v1);

    //accessing values from vector

    let value_1 = v1[0];
    let value_2 = v1.get(1);//if specified index is out of range, it will return none
    let value_3 = &v1[2];


    println!("{}, {}", value_1, value_3);
    println!("{:?}", value_2);
    

}
