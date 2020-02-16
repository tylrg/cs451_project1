use std::env;
//use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;

//main method duh
fn main() -> std::io::Result<()> {
    
    let args: Vec<String> = env::args().collect();//reading in arguments
    
    //pattern matching for number of arguments
    match args.len() {
        1=>{
            eprintln!("Need to have at least one argument")//reject
        },
        2=>{
            read_byte_by_byte(&args[1])?;//read the message in the specidifed file
            return Ok(());
        },
        3=>{
            write_message(&args[1],&args[2])?;//hide the messages in the specified file
            return Ok(());
        },
        _=>eprintln!("Need to have at most two arguments"),//default case
    }

    Ok(())//got to the end of the file and yeah it might work
}

fn read_byte_by_byte(path: &str)-> Result<Vec<u8>,io::Error>{
    
    let mut f = fs::File::open(path)?;//establishing variables necessary for method
    let mut bytes = vec![0u8,0];//vector of all of the bytes
    let mut byte_buffer = [0u8,8];//bytes being read in

    //looping through input file and reading bytes into buffer and then adding to bytes
    loop{
        match f.read(&mut byte_buffer)?{
            num_bytes_read if num_bytes_read == 0 =>{
                break;
            },
            num_bytes_read if num_bytes_read > 0 =>{
                bytes.extend(&byte_buffer[..num_bytes_read]);
            },
            _ =>{
                panic!("Failed to read in bytes!"); 
            }
        }
    }



    let mut binary_values = vec![0u8,0];//binary values to print, can remove later?
    let mut message = vec![0 as char,'0'];//character form of the binary values
    binary_values.pop();//idk why but these arrays have two elements
    binary_values.pop();
    message.pop();
    message.pop();
    let mut index = 0;
    let mut binary:u8 = 0000_0000;//default binary value
    let mut nl_count = 0;//number of newlines found in header

    
    for &byte in &bytes{
        if nl_count >=3{
            if byte%2 != 0 {binary = set_bit(binary,index);}//if the byte is odd, set the corresponding bit to 1
            index+=1;//increment the index of which bit we are looking at

            if index%8 == 0{
                binary_values.push(binary);
                let ch = binary as char;
                message.push(ch);
                if binary == 0{
                    println!("Message :{:?}",message);
                    println!("Binary Values: {:?}",binary_values);
                    break;
                }
                index=0;
                binary=0;
            }
        }   

        //checking if the byte is equal to newline
        if byte == 10 {nl_count+=1;}
    }

    Ok(bytes)
}

//takes in a string literal name of the file and a string literal message
fn write_message(path: &str,message: &str)-> Result<Vec<u8>,io::Error>{
    
    let mut f = fs::File::open(path)?;
    let mut bytes = vec![0u8,0];//vector of all of the bytes
    let mut byte_buffer = [0u8,8];//bytes being read in

    let mut message_bytes = vec![0u8,0];
    let mut binary_string = vec![0u8,0];
    message_bytes.pop();
    message_bytes.pop();
    binary_string.pop();
    binary_string.pop();
    for x in message.chars(){
        print!("{} ",x);
        let message_binary = x as u8;
        message_bytes.push(message_binary);
        println!("{}",message_binary);
    }
    //looping through input file and reading bytes into buffer and then adding to bytes
    loop{
        match f.read(&mut byte_buffer)?{
            num_bytes_read if num_bytes_read == 0 =>{
                break;
            },
            num_bytes_read if num_bytes_read > 0 =>{
                bytes.extend(&byte_buffer[..num_bytes_read]);
            },
            _ =>{
                panic!("Failed to read in bytes!"); 
            }
        }
    }

    // for byte in &mut bytes{
    //     *byte = 0u8;
    // }

    let mut nl_count:u8 = 0;
    let mut input_index:u8 = 0;
    let mut message_index:u8 = 0;

    println!("Length of Input String: {}",message_bytes.len());
    println!("Message bytes: {:?}",message_bytes);
    
    for byte in &mut bytes{
        //bytes is the array
        //byte is the current byte we are reading in from the array
        //message index is the current character that is being indexed

        if nl_count>=3 {
            if message_index == message_bytes.len() as u8 {break;}
            let overwrite = message_bytes[message_index as usize];
            println!("{:?} to {:?}",byte,overwrite);
            
            //input char is even
            let overwrite_val = check_bit(overwrite,message_index);
            let o_string = format!("{:b}",overwrite_val);
            println!("{}",o_string);

            if overwrite_val==0{
                //current byte is odd
                if *byte %2 != 0{
                    println!("Doing something");
                }
                else{println!("Doing nothing");}
            }
            else if overwrite > 0{
                //current byte is even
                if *byte%2==0{
                    println!("Doing something");
                }
                else{println!("Doing nothing");}
            }





            message_index+=1;

            //done with a character
            if message_index == 8{
                message_index=0;
                input_index+=1;

            }
            println!("\n");
        }
        if *byte == 10 {nl_count+=1;}
    }


    Ok(bytes)
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
fn check_bit(byte: u8,position: u8)-> u8{
    println!("Checking: {}",byte);
    match position{
        0 => byte & 0b1000_0000,
        1 => byte & 0b0100_0000,
        2 => byte & 0b0010_0000,
        3 => byte & 0b0001_0000,
        4 => byte & 0b0000_1000,
        5 => byte & 0b0000_0100,
        6 => byte & 0b0000_0010,
        7 => byte & 0b0000_0001,
        _ => panic!("Uh, dude, what are you doing?")
    }
}
