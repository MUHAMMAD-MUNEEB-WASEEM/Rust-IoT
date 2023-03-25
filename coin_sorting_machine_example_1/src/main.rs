
//3
//we create instance of enum of penny coin, we passed that instance to our function which matches
//my coin value with all coins and it matches with penny and return 1, and then we print that 1
fn main() {
    let my_coin = Coin::Quarter(USstate::Alalaska);//using alalaska variant of Quarter
    let from_penny_to_cents = usa_coin_to_cents(my_coin);
    
    println!("{}", from_penny_to_cents);
}

#[derive(Debug)]

//1
//first we define 4 coin types use in USA with the help of enum

enum Coin{
    Penny, 
    Nickle,
    Dime, 
    Quarter(USstate),           //quarter coin has further 50 variants in US, we have defined 2 variants
}                               //in below enum and call them here

#[derive(Debug)]

enum USstate{                  //defining the variants of quarter, we have defined 2 only out of 50
    Alabama,
    Alalaska,
}

//2
//Then we create function which takes intance of enum or particular provided coin as parameter and 
//match whether that particular instance coin matches with penny, nickle, dime or quarter and return
//integer value thats why we have mentioned u8 return. 

fn usa_coin_to_cents(coin: Coin) ->u8{

    match coin {   

    Coin::Penny => {
        println!("Lucky penny");//if we want to assing more value, then we use {}
        1
    },
    Coin::Nickle => 5,
    Coin::Dime => 10, 
    Coin::Quarter(state) => {       //assigning variants
        println!("{:#?}", state);
        25
},
}
}