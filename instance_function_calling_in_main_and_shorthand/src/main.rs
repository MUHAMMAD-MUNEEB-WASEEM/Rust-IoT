#[derive(Debug)]
struct Book
{
    name: String,//while creating struct we define values as the type of data which we will lateruse
    author: String,
    price: u16,
    availability: bool,
} //Its like a template which we will use in main function


fn main() 
{

    let book_1 = build(String::from("Book AA"), String::from("Author AA"));
    //calling function that contains instance of struct book
    println!("\n new book \n{:#?}", book_1);//printing book_1

}
fn build (name:String, author:String) -> Book
//defining function that has instance of struct which we later call in main function
//this function take two parameters, name and author
{
    //Without using shorthand

    //  Book{//using user or custom defined book data type 
    //     name: name//without shorthand
    //     author: author
    //     price: 500,
    //     availability: true,
    //  }

    //With using shorthand

    Book{//using user or custom defined book data type 
        name,//with shorthand
        author,
        price: 500,
        availability: true,
     }
}
