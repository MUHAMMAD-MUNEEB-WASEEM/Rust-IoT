fn main() {
    let mut counter = 0;
    while counter < 3//This while loop runs three times
    {
        println!("Hello World");
        counter = counter + 1;//increment counter value
    }

    //Accessing array elements using while loop

    println!("\nAccessing array elements using while loop\n");

    let numbers = [1, 23, 45, 56, 67, 78, 89];
    let mut counter_1 = 0;
    while counter_1 < numbers.len()
    {
        println!("{}", numbers[counter_1]);
        counter_1 = counter_1 + 1;
    }
}
