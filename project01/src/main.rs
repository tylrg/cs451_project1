use std::env;
use std::fs::File;
use std::io;
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
    println!("{}",contents);

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

// fn read_byte_by_byte(path: &str)->Result<Vec<u8>, io::Error>{
//     Ok()
// }

fn set_bit(byte: u8, position: u8)->u8{
    match position{
        0 => byte | 0b1000_0000,
        1 => byte | 0b0100_0000,
        2 => byte | 0b0010_0000,
        3 => byte | 0b0001_0000,
        4 => byte | 0b0000_1000,
        5 => byte | 0b0000_0100,
        6 => byte | 0b0000_0010,
        7 => byte | 0b0000_0001,
        _ => panic!("Uh, dude, what are you doing?")
    }
}
fn unset_bit(byte: u8,position: u8)-> u8{
    match position{
        0 => byte & 0b0111_1111,
        1 => byte & 0b1011_1111,
        2 => byte & 0b1101_1111,
        3 => byte & 0b1110_1111,
        4 => byte & 0b1111_0111,
        5 => byte & 0b1111_1011,
        6 => byte & 0b1111_1101,
        7 => byte & 0b1111_1110,
        _ => panic!("Uh, dude, what are you doing?")
    }
}
fn toggle_bit(byte: u8, position: u8)->u8{
    match position{
        0 => byte ^ 0b1000_0000,
        1 => byte ^ 0b0100_0000,
        2 => byte ^ 0b0010_0000,
        3 => byte ^ 0b0001_0000,
        4 => byte ^ 0b0000_1000,
        5 => byte ^ 0b0000_0100,
        6 => byte ^ 0b0000_0010,
        7 => byte ^ 0b0000_0001,
        _ => panic!("Uh, dude, what are you doing?")
    }
}
