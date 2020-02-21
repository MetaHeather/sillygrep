use std::env;
fn main() {
    //use collect to turn the iterator into a vector containing all the values
    //produced by the iterator
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
