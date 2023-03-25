fn main() {
    let result1 = gives_ownership();// that return heap string s will store in result1 variable 
    //and it has its ownership
    println!("{}", result1);//proving ownership
    
    let s1 = String::from("Universe");//s1 comes into scope
    let result2 = take_and_gives_ownership_to_other_variable(s1);//s1 transfer its ownership to result 2
    //result2 comes into scope and has ownership of "universe"

    println!("{}", result2);//proving ownership
    //println!("{}", s1);//it gives an error as value moved to result2 and it does not have an ownership
}//result1 out of scope
//result2 out of scope


fn gives_ownership() -> String //this function return heap string s
{
    let s = String::from("hello");
    s
}

fn take_and_gives_ownership_to_other_variable(x:String)->String
//this function takes heap string and return its ownership to some other variable
{
    x
}