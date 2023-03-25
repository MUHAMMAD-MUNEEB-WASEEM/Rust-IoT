fn main() {
    let numbers = [1, 23, 45, 67, 89];
    // OR
    // let numbers: [u32;5] = [1, 23, 45, 67, 89];
    // here in square brackets ;5 represents length of array so instead of writing u32 5 times
    // we can use it with the required length [u32;5] is representing u32 5 times

    println!("Printing complete array {:#?}", numbers);

    //If we have same data within an array then we can also define it as
    let same_num = [5;8];
    //5 is the data of same_num array where 8 is its lenght means 5 appears 8 times

    println!("{:#?}", same_num);

    //Accessing array element
    println!("{}", numbers[2]);

}
