fn main() {
    let mut v = vec![10, 20, 30, 40];

    for i in &mut v{                    //to change i value, use mut
        println!("{}", i);
        *i += 20;
        println!("adding 20->{}", i);
    }
}
