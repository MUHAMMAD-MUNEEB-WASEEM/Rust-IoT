fn main() {
    let a: u8 = 10;
    
    let b = &a;//pointer b, address of a
    let c = &b;//pointer c, address of b

    println!("a : {}, b : {}, c : {}", a, b, c);
    //when we print b, it wiil also print 10 as b has address of a and when we print b, it 
    //it has address of a and then it goes their and print value 10

    //Similary when we print c, it wiil also print 10 as c has address of b and b has address of a
    //and when we print c, it goes to b to a and print value 10

    //to print address, we have to use :p
    println!("{:p}, {:p}", b, c);
}
