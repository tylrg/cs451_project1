use std::env;
use std::fs::File;
use std::io::prelude::*;

//main method duh
fn main() -> std::io::Result<()> {
    //reading in arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    
    //pattern matching for number of arguments
    match args.len() {
        1=>{
            eprintln!("Need to have at least one argument")//reject
        },
        2=>{
            find_message(&args[1]);//find the message in the specified file
        },
        3=>write_message(&args[1],&args[2]),//hide the messages in the specified file
        _=>eprintln!("Need to have at most two arguments"),//default case
    }

    //code for testing how to open and read from file. Need to do byte by byte reading
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //println!("{}",contents);

    Ok(())//got to the end of the file and yeah it might work
}

//takes in a string literal name of the file
fn find_message(filename: &str){
    println!("Finding the message in the file {}",filename)
}

//takes in a string literal name of the file and a string literal message
fn write_message(filename: &str,message: &str){
    println!("Embedding the message {} in the file {}",message,filename)
}

fn read_as_byte(){

}
