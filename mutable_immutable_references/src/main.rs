fn main() {

    // let mut s = String::from("Hello");
    // let b = &s; -> no problem
    // let c = &s; -> no problem
    // let d = &s; -> no problem
    // let a = &mut s; ->problem

    // println!("{}, {}, {}, {}", a, b, c, d);

    //The above code will raise an error, we have multiple immutable references of same data which is 
    //fine, but problem is that the scope of b, c, d does not end yet and we have use mutable reference
    //which confuses compiler, as one pointer allows only reading while other allowing for writing

    let mut s = String::from("Hello");
    let b = &s;
    let c = &s;
    let d = &s;
    println!("{}, {}, {}", b, c, d);
    //Now this time the scope of b, c, and d is end as we know the scope of pointers end their 
    //they were last used, so here b, c, and d are used for the last time

    let a = &mut s;//now defining mutable reference
    a.push_str(" World");//changing to pointing address
    println!("{}", a);


}
