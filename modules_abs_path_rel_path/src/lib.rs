mod front_house{
    pub mod hosting{
        pub fn add_to_waitlist(){

        }
    }
}
pub fn eat_at_restaurant(){
    
    //absolute path
    crate::front_house::hosting::add_to_waitlist();

    //relative path
    //both eat at restaurant function and front house module are at same level
    front_house::hosting::add_to_waitlist();
}