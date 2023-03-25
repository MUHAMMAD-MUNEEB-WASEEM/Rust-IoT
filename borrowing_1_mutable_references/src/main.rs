fn main() {
    let mut s = String::from("Hello People");//mutable 
    changing(&mut s);//passing mutable reference
}
fn changing(x:&mut String) 
//we are adding some value to string, but we dont have its ownership, we are just borrowing it, so we 
//cannot add any thing to, to do this we have to make its reference mutable, so it can accept changing
{
    x.push_str(" of World");
    println!("{}", x);

}