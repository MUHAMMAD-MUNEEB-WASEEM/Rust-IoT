fn main() {
    
    //defining string heap
    let mut s_heap = String::from("hello");
    println!("{}", s_heap);

    //converting string literal to string heap using to_string function
    let s_literal = "foo";
    let s_literal_to_s_heap = s_literal.to_string();
    println!("{}", s_literal_to_s_heap );

    //creating new variable that have ownership of s_heap
   let mut s_heap1 = &mut s_heap;
    
   //pushing string type to s_heap using push_str
   s_heap1.push_str(" hi");
   println!("{}", s_heap1);

   //pushing char to s_heap using push
    s_heap1.push('o');
    println!("{}", s_heap1);

    


}
