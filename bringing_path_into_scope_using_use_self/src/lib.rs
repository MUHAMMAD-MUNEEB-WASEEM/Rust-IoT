pub mod front_house{
    pub mod hosting{
        pub fn add_to_waitlist(){

        
    }
}
}
//Abs path

use crate::front_house::hosting;        //bringing abs path into scope with the help of use
pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();         //now its not necessary to define complete path every time
    hosting::add_to_waitlist();
}

 //Relative Path


// use self::front_house::hosting;        //bringing relative path into scope with the help of use and self
// pub fn eat_outside_restaurant(){
//     hosting::add_to_waitlist();         //now its not necessary to define complete path every time
//     hosting::add_to_waitlist();
// }