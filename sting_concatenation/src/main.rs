fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //first method, using format macro

    let s_result1 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s_result1);

    //second method, using +, but remember that we use & with all strings other than first string

    let s_result2 = s1+ "-" + &s2 + "-" + &s3;
    println!("{}", s_result2);

    
   

}
