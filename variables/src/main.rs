fn main() {
    let temperature = 35;
    // let is used to declare variable so let temperature declares temperature variable
    // = is used to initialize variable so let temperature = initialze temperauture variable with value 35
    println!("Temperature value is {}", temperature);

    // Declaring mutable variable so we can change its value throughout the program

    let mut speed = 50;// use mut after let and before variable name
    println!("I initialized speed to be {} ", speed);

    //let change speed variable value
    speed = 80;
    println!("Modifying speed variable value {}", speed);

}
