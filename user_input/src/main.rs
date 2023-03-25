use std::io;
// use ->keyword, std->standard crate, io-> library
//So in short, use io library from standard crate

fn main()
{
    let mut s = String::new();//new is an associated function of type String
                            // new returns empty string which will be save in s
                            //we make s mutable so we can take mulitple inputs

    //taking input
    io::stdin().read_line(&mut s)//io::stdin() takes input and stores it in variable s
        .expect("Failed to read a line");//readline returns a type which has two results either "ok" or "error", ok means you have enter correct input where to treat an error we use expect part

    //println!("{}", s);//printing user input
    
    //variable takes input by default in string as we have use String type, so if enter int it will also count it as string so we cannot perform any mathematical operations to it
    //for that we have to convert that input to int
    
    let mut s_integer: u32 = s.trim().parse()//trim function remove any of the whitespaces from both left and right, now parse is used for type conversion, we ues parse() and type annotation u32 means we have to parse our string into type int(u32) and save that result in mut variable s_integer
        .expect("Please enter a digit");//like readline parse()also returns a type with two results, "ok" and "error"
    
    s_integer = s_integer + 2;//now our string input which was stores in s converted into int(u32) and we have added 2 to it to get surity by performing mathematical operations to it
    
    println!("{}", s_integer);//printing result

//conclusion
//use standard crate library
//use mut variable to store string input
//take input from user and stores it in that variable and print that input

//***To convert string input to some other type***

//first we have to apply trim function to string input to remove any whitespaces from it
//then use parse() function to convert that string input to some other type with the help of type annotation
//then stores it in some new variable
}


