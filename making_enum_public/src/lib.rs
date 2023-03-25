#![allow(dead_code)]
#![allow(unused_variables)]

mod front_house{
    pub enum Appetizer{                         //making public enum
        Soup, 
        Salad,
    }
}
fn eat_at_restaurant(){
    let meal1 = front_house::Appetizer::Soup;   //calling public enum as eat at restaurant and front house are at same level
    let meal2 = front_house::Appetizer::Salad;
}