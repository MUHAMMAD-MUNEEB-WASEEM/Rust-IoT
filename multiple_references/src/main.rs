fn main() {
    let mut s = String::from("Hello People");

    //creating different scopes for multiple references

    {
        let a = &mut s;//first reference
        a.push_str(" of World");//when changing here, it will also reflect in bS
        println!("{}", a);
    }
    
    { 
        let b = &mut s;//second reference of same s, so leads to multiple references
        println!("{}",b);
    }
    

}
    
