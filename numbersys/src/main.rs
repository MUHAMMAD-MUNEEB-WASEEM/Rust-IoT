fn main() {
    // binary - base = 2 - 0,1 - syntax = 0b
    // octal - base = 8 - 0,1,2,3,4,5,6,7 - syntax = 0o
    // decimal - base = 10 - 0,1,2....,9  - syntax = any number
    // hexadecimal - base = 16 - 0,1,2,....,9,A,B,.....,F - syntax = 0x
    //byte is used to tell ASCII value of any character - syntax = b'A' or b'B' etc.

    let binary_num = 0b10101;
    let octal_num = 0o66;
    let decimal_num = 150;
    let hexadecimal_num = 0xff;
    let ascii_value = b'A';
    println!("{}", binary_num);
    println!("{}", octal_num);
    println!("{}", decimal_num);
    println!("{}", hexadecimal_num);
    println!("{}", ascii_value);

}   
