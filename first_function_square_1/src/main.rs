//Making a program in which function returns value

fn main() {
    println!("\nMaking program in which function returns values\n");
    println!("Calling square function");
    
    //calling function in main part
    //As functions returns a value, we have to store it in some variables

    let (square_of_x, square_of_y) = square(2, 3.1);//arguments, square of x stores in square_of_x 
    
    //and square of y stores in square_of_Y
    //printing values

    println!("{}, {}", square_of_x, square_of_y);
}

//defining function after main part

fn square(x: u32, y:f64) -> (u32, f64)// return int and float, parameters with type for feasibility
{

    let result_1 = x * x;//statements, square of x
    let result_2 = y * y;//square of y

    (result_1, result_2)//expressions, returning 2 values

}
