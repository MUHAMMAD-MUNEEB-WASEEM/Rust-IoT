mod front_house{
    pub mod hosting{
        pub fn add_to_waitlist(){

        }
    }
}
mod dining{
pub fn eat_at_restaurant(){
    
    //absolute path
    crate::front_house::hosting::add_to_waitlist();

    //relative path
    //here  eat at restaurant function and front house module are not at same level, we have used super
    //here which takes us to inner or lower module to crate and from their we can list required modules.
    
    super::front_house::hosting::add_to_waitlist();
}
}