fn main() {

    //looping over string using chars() method
   for c in "muneeb".chars(){
       println!("{}", c);
   }

   println!("\n");

   //getting utf code of each character in a string using bytes method
   for a in "muneeb".bytes(){
       println!("{}", a);
   }
}
