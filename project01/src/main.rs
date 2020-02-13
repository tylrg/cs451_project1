use std::env;
//use std::fs::File;
use std::fs;
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
            //find_message(&args[1]);//find the message in the specified file
        },
        3=>write_message(&args[1],&args[2]),//hide the messages in the specified file
        _=>eprintln!("Need to have at most two arguments"),//default case
    }

    let _ret_val = read_byte_by_byte(&args[1])?;    
    Ok(())//got to the end of the file and yeah it might work
}

fn read_byte_by_byte(path: &str)-> Result<Vec<u8>,io::Error>{
    println!("File path: {}",path);
    let mut f = fs::File::open(path)?;
    let mut bytes = vec![0u8,0];
    let mut byte_buffer = [0u8,8];

    //let mut i = 0;
    loop{
        match f.read(&mut byte_buffer)?{
            num_bytes_read if num_bytes_read == 0 =>{
                break;
            },
            num_bytes_read if num_bytes_read > 0 =>{
                bytes.extend(&byte_buffer[..num_bytes_read]);
                //println!("{:x?} {:x?}",&byte_buffer[..1],&byte_buffer[1..num_bytes_read]);
            },
            _ =>{
                panic!("What the fuck is happening"); 
            }
        }
        //i+=1;
    }


    //i = 0;
    // let binary:u8 = 0000_0000;
    // let s = format!("{:b}",binary);
    //println!("{}",s);
    // for byte in &bytes{
    //     print!("{:x?} ",byte);
    //     if i%8 == 0{
    //         println!("");
    //     }
    //     i+=1;
    // }

    let mut binary_values = vec![0u8,0];
    let mut index = 0;
    let mut binary:u8 = 0000_0000;
    //let s = format!("{:b}",binary);
    for byte in &bytes{
        if byte%2 != 0 {
            binary = set_bit(binary,index);
        }

        //print!("{:x?} ",byte);
        index+=1;
        if index%8 == 0{
            binary_values.push(binary);
            let s = format!("{:b}",binary);
            let ch = binary as char;
            let hex = format!("{:x}",binary);
            println!("{} {} {}",s,hex,ch);
            index=0;
            binary=0;
        }
    }
    

    Ok(bytes)
}

//takes in a string literal name of the file
// fn find_message(filename: &str){
//     println!("Finding the message in the file {}",filename);
// }

//takes in a string literal name of the file and a string literal message
fn write_message(filename: &str,message: &str){
    println!("Embedding the message {} in the file {}",message,filename)
}



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
// fn unset_bit(byte: u8,position: u8)-> u8{
//     match position{
//         0 => byte & 0b0111_1111,
//         1 => byte & 0b1011_1111,
//         2 => byte & 0b1101_1111,
//         3 => byte & 0b1110_1111,
//         4 => byte & 0b1111_0111,
//         5 => byte & 0b1111_1011,
//         6 => byte & 0b1111_1101,
//         7 => byte & 0b1111_1110,
//         _ => panic!("Uh, dude, what are you doing?")
//     }
// }
// fn toggle_bit(byte: u8, position: u8)->u8{
//     match position{
//         0 => byte ^ 0b1000_0000,
//         1 => byte ^ 0b0100_0000,
//         2 => byte ^ 0b0010_0000,
//         3 => byte ^ 0b0001_0000,
//         4 => byte ^ 0b0000_1000,
//         5 => byte ^ 0b0000_0100,
//         6 => byte ^ 0b0000_0010,
//         7 => byte ^ 0b0000_0001,
//         _ => panic!("Uh, dude, what are you doing?")
//     }
// }
