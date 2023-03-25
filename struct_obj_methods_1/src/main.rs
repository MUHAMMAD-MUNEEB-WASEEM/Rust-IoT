//defining struct
struct Rectangle
{
    height: u32,
    width: u32,
}

//methods for struct rectangle
impl Rectangle
{
    // takes other rectangle attributes and compare them with rectangle used as object at that time 
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

//creating objects
fn main ()
{
    let rec1 = Rectangle{height:100, width:50};
    let rec2 = Rectangle{height:90, width:40};
    let rec3 = Rectangle{height:110, width:60};
    println!("Rec1 can hold Rec2 or not : {}", rec1.can_hold(&rec2));//comparing rec1 and rec2 attributes
    println!("Rec1 can hold Rec3 or not : {}", rec1.can_hold(&rec3));//comparing rec1 and rec3 attributes
}