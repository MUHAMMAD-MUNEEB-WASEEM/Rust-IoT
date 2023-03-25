fn main() {
    for a in 0..5//0 till 5 where 5 not included
    {
        println!("{}", a);
    }

    println!("\nPrinting numbers in reverse order\n");

    for a in (0..5).rev()// used reverse function to get numbers in reverse order
    {
        println!("{}", a);
    }

    println!("\nPrinting Hello world 5 times\n");


    for a in 0..5
    {
        println!("{}. Hello World", a);
    }

    println!("\nAccessing Array elements\n");

    let numbers = [1, 23, 45, 67, 89];

    for num in numbers.iter()//iter iterate over array elements one by one 
    {
        println!("{}", num);
    }
}

