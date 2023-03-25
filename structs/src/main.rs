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
    let book_1 = Book{//using user or custom defined book data type 
        name: String::from("Book A"),
        author: String::from("Author A"),
        price: 500,
        availability: true,
    };//semi colon as defining variable

    let book_2 = Book{
        name: String::from("Book B"),
        author: String::from("Author B"),
        price: book_1.price,//using values from book 1
        availability: book_1.availability,
    };

    let book_3 = Book{
        name: String::from("Book C"),
        author: String::from("Author C"),
        ..book_1//this make rest values similar to book 1
    };
    println!("{:#?}", book_1);
    println!("{:#?}", book_2);
    println!("{:#?}", book_3);

}
