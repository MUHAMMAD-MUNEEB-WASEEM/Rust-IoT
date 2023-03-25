fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    //Now this time both s1 and s2 have different heap data, s1 has hello and s2 also has hello but both
    //s1 and s2 not pointing to same hello, both pointing to different hello and this is deep copy
    //where we have copied both stack and heap data

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
