fn main() {
    let s = String::from("Hello");//s in the scope
    
    //we can get perform operation on data in heap memory and maintains its ownership with the help of tuple
    let (result1, result2) = length_and_ownership(s);// s out of scope
    
    //result 1 in the scope and has ownership of len of s
    //result 2 in the scope and has ownership of s
    
    println!("The lenght of {} is {}", result2, result1); 

}//result 1 and result 2 out of the scope

fn length_and_ownership(name:String) ->(usize, String)
// it return length and the same input string so it can be used whenever wants

{
    (name.len(), name)
}