#[derive(Debug)]
//defining struct student 
struct Student
{
    name: String,
    grade: String,
    age: u32,
    percentage: u32,
}

//making implementation block consisted of method and associated function
impl Student {
    fn child(name: String, grade: String, age: u32, percentage: u32) -> Student {
         Student{
    name,
    grade,
    age,
    percentage,
}
    
}
    fn percentage(&self) {
        println!("{}",self.percentage);
    }

}

//main function
fn main()
{
    let child_1 = Student::child(String::from("Muneeb"), String::from("A"), 20, 80);//instance from associated function
    println!("{:#?}", child_1);
    child_1.percentage();//calling method using instance
}