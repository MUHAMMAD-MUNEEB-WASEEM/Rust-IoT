

fn main(){
 
//2
    let four = Some(4);                         //pass 4 in Some
    let result1 = value_match(four);            //Pass some(4) as argument to value_match function
    println!("{:#?}", result1);                 // it returns 5 as Some matches to Some part

    let none_type = None;                       //Define none type
    let result2 = value_match(none_type);       //pass it to value match function    
    println!("{:#?}", result2);                 //it matches with None and return None


} 

//1
//Making a function that takes Some and None type as paramater and matches whether given parameter 
//belongs to None or Some, and based on that return answer in Some or None

fn value_match(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,                   //Remembet while matching None and Some type, they should be   
        Some(i) => Some(i+1),           //they both should be checked while checking, it is not 
    }                                   //that you should only check None or Some, they both should
}                                       //checked side by side, otherwise raise an error of pattern
                                        //not covered