#[derive(Debug)]


enum Student{                   //enum is used where we want to categorized the data, like we can make
                                // student data easily using struck, but we cannot classified them in
                                //online and onsite categories, so for that we have to used enum
    Onsite, 
    Online
}
fn main() {
    let student1 = Student::Onsite;//accessing onsite variant of student enum
    let student2 = Student::Online;
    println!("{:#?}", student1)
}
