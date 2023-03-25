fn main() {
    
    let student = (20, 'A', 80.5);
    //or let student: (u32, char, f64) = (20, 'A', 80.5);
    println!("Using Structuring Approach");

    println!("{}", student.0);
    println!("{}", student.1);
    println!("{}", student.2);

    println!("\nUsing Destructuring Approach\n");

    let (x, y, z) = (20, 'A', 80.5);

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!("\nPrinting complete tuple\n");
    // to print complete tuple we have to place :#? within curly braces so {:#?} used
    // to print complete tuple or array
    println!("{:#?}", student);
    
}
