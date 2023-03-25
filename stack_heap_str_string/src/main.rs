fn main() {
    //Defining heap
    
    // Using string type String as it belongs to heap
    //variable(owner) = value

    let mut a = String::from("Hello People");//allocate memory in heap
    a.push_str(" of World");//adding to heap proving its dynamic size
    println!("{}", a);

    //Defining stack

    //Using string literal &str as it belongs to stack
    
    // let mut m = "Hello People";
    // m.push_str(" of World");//adding to stack which given an error proving its fixed size
    // println!("{}", m);
    //Error: method not found in `&str`//returning an error confirming stack is of fixed size

}//owner out of scope after this bracket so drop value and clean up the memory both in stack and heap

// a is drop now(heap)
// m is drop now(stack)
