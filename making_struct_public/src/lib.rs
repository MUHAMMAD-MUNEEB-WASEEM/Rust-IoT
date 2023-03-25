pub mod front_house{                           //1: making front house module
    pub struct Breakfast{                      //2: making public struct so we can do changes to it
        pub toast:String,
        seasonal_fruits:String,                //3: we have not assigned pub keyword with it,means we cannot do any changes in  it
    }


    impl Breakfast{                            //4: making method of struct with one pub function takes user required toast value in string as parameter and assigned that to toast kind
         pub fn new(toast: String)->Breakfast{
            Breakfast{
            toast:toast,                           //or simplhy toast
            seasonal_fruits:String::from("oranges")//5:private: so it cannot be changed and it is predefined
    }
}
}
}                                                  // front house module ends

fn eat_at_restaurant(){                            //6: making function that assigned struct to meal 
    let meal = front_house::Breakfast::new(String::from("Wheat"));//as both eat at restaurant and front house are at same level, so we use relative path
}
