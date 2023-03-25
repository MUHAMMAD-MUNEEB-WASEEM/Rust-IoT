fn main() {
    //Stack memory 

    let x = 5;
    let y = x;//copying value of x to y
    println!("{}, {}", x, y);//copy done on stack

    //heap memory

    let s1 = String::from("hello");//allocating heap memory
    //move occurs because `s1` has type `std::string::String`, which does not implement the `Copy`trait
    
    let s2 = s1;//only stack(meta data like pointer, len, capacity) part move(shallow copy) to s2
    //value moved here
    //both stack and heap data would not be move(no deep copy)
    
    //lets try to print s1
    //println!("{}", s1);
    //Error: value borrowed here after move

    //Explanantion:

    //Earlier we have studied 3 ownership rules where rule 2 was value every has only one owner at time,
    //but in our case s1 and s2 both pointing to hello means hello has 2 owners s1 and s2
    //and when last curly bracket end we will also get "double free error" because both s1 and s2
    // try to free the same memory same time, so what rust do here when we copy(move) s1 data to s2
    //s1 lose its ownership to hello and now only s2 will have the ownership of hello and we also got
    //the confirmation when we print s1 rust gave an error that the value have been move so when we
    //print s2 it gives hello as only s2 has the ownership of hello and in this way we will also not
    //get the double free error as only s2 will free its memory when drop function called afer last }

    println!("{}", s2);//only s2 is the owner of hello
}
