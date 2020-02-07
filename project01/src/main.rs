use std::env;


fn main() {
    println!("Hello, wold!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
