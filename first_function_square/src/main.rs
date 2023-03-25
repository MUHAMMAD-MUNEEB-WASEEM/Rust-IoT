fn main() {
    println!("Calling square function");
    //calling function in main part
    square(2, 3.1);//arguments
    
}

//defining function after main part
fn square(x: u32, y:f64)//parameters, with type for feasibility
{

    let result_1 = x * x;//square of x
    let result_2 = y * y;//square of y

    println!("The square of {} is {}", x, result_1);
    println!("The square of {} is {}", y, result_2);

}

