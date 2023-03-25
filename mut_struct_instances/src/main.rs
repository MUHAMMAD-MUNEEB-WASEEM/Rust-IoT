#[derive(Debug)]
struct Book
{
    name: String,//while creating struct we define values as the type of data which we will lateruse
    author: String,
    price: u16,
    availability: bool,
} //Its like a template which we will use in main function


fn main() 
{   //creating instance of struct book by using struct name followed by curly braces and key value pairs
    let mut book = Book{//using user or custom defined book data type 
        name: String::from("Book A"),
        price: 500,
        availability: true,
        author: String::from("Author A"),//changing key value pair order from template does not matter

    };//semi colon as defining variable

    println!("old book \n{:#?}", book);
    
    book.name = String::from("Book double A");
    
    println!("\n new book \n{:#?}", book);

}
