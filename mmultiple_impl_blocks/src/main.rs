struct Rectangle{
    height: u32,
    width: u32,
}
// all methods related to single struct or data type should be same impl block
impl Rectangle {
    fn area(&self) ->u32//first method, returns area of type u32
    {
        self.height*self.width 
    }

    fn can_hold(&self, other:&Rectangle) -> bool {//returns bool(true/false)
        if self.height > other.height && self.width > other.width//comparing obj attributes with other rectangle attributes
        {
            true
        }
        else
        {
            false
        }
    }

}

//multiple impl block of same struct or data type is not recommended
// impl Rectangle {
//         fn area(&self) ->u32//first method, returns area of type u32
//         {
//             self.height*self.width 
//         }
//     }

// impl Rectangle{   
//         fn can_hold(&self, other:&Rectangle) -> bool {//returns bool(true/false)
//             if self.height > other.height && self.width > other.width//comparing obj attributes with other rectangle attributes
//             {
//                 true
//             }
//             else
//             {
//                 false
//             }
//         }
    
// }
fn main()
{

    let rec = Rectangle{height:5, width:4};//creating obj or instance
    let result = rec.area();//calling method using object
    println!("The area of rectangle is {}", result);

    let rec1 = Rectangle{height:100, width:50};
    let rec2 = Rectangle{height:90, width:40};
    let rec3 = Rectangle{height:110, width:60};
    println!("Rec1 can hold Rec2 or not : {}", rec1.can_hold(&rec2));//comparing rec1 and rec2 attributes
    println!("Rec1 can hold Rec3 or not : {}", rec1.can_hold(&rec3));//comparing rec1 and rec3 attributes
}
