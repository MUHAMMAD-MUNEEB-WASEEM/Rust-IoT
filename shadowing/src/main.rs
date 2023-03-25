fn main() {
    let x = 4;
    println!("Initializing x to {}", x);

    let x = x * 2;
    // compiler runs left to righ
    //8 = 4 * 2
    println!("Modify x value using shadowing, now x = {}", x);

    //changing data type of x
    let x:f64 = 4.2;
    println!("{}", x);

}
