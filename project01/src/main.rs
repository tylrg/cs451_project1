use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

//main method
fn main() -> std::io::Result<()> {
    
    let args: Vec<String> = env::args().collect();//reading in arguments
    
    //pattern matching for number of arguments
    match args.len() {
        1=>{
            eprintln!("Need to have at least one argument: (path to input file) or (path to input file, path to message to hide)");//reject
        },
        2=>{
            read_message(&args[1])?;//read the message in the specidifed file
            return Ok(());
        },
        3=>{
            write_message(&args[1],&args[2])?;//hide the messages in the specified file
            return Ok(());
        },
        _=>eprintln!("Need to have at most two arguments"),//default case
    }

    Ok(())//return
}

//takes in a string literal name of file
fn read_message(path: &str)-> Result<Vec<u8>,io::Error>{
    
    let mut f = fs::File::open(path)
        .expect("Something went wrong reading the file");//establishing variables necessary for method
    let mut bytes = vec![0u8,0];//vector of all of the bytes
    let mut byte_buffer = [0u8,8];//bytes being read in

    //looping through input file and reading bytes into buffer and then adding to bytes
    loop{
        match f.read(&mut byte_buffer)?{
            num_bytes_read if num_bytes_read == 0 =>{
                break;//if no more bytes can be read, break
            },
            num_bytes_read if num_bytes_read > 0 =>{
                bytes.extend(&byte_buffer[..num_bytes_read]);//read in bytes if they can be
            },
            _ =>{
                panic!("Failed to read in bytes!"); 
            }
        }
    }

    let mut binary_values = vec![0u8,0];//binary values to print, can remove later?
    //let mut message = vec![0 as char,'0'];//character form of the binary values
    binary_values.pop();//idk why but these arrays have two elements
    binary_values.pop();
    //message.pop();
    //message.pop();
    let mut index = 0;
    let mut binary:u8 = 0000_0000;//default binary value
    let mut nl_count = 0;//number of newlines found in header

    //looping through all of the bytes
    for &byte in &bytes{
        //checking if we are in pixel data
        if nl_count >=3{
            if byte%2 != 0 {binary = set_bit(binary,index);}//if the byte is odd, set the corresponding bit to 1
            index+=1;//increment the index of which bit we are looking at

            //if a byte has been completed
            if index%8 == 0{
                binary_values.push(binary);
                //let ch = binary as char;
                //message.push(ch);
                if binary == 0{
                    break;
                }
                index=0;
                binary=0;
            }
        }   

        //checking if the byte is equal to newline, trying to find pixel data
        if byte == 10 {nl_count+=1;}
    }

    //adding a newline character and printing to stdout
    binary_values.push(10);
    io::stdout().write(&binary_values)
        .expect("Something went wrong writing to the file");

    Ok(bytes)
}

//takes in a string literal name of file and a string literal path to message file
fn write_message(path: &str,filename: &str)-> Result<Vec<u8>,io::Error>{
    
    let message = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut f = fs::File::open(path)
        .expect("Something went wrong reading the file");
    let mut bytes = vec![0u8,0];//vector of all of the bytes
    bytes.pop();//need to pop two off for some reason?
    bytes.pop();
    let mut byte_buffer = [0u8,8];//bytes being read in

    let mut message_bytes = vec![0u8,0];//values of chars as number
    message_bytes.pop();
    message_bytes.pop();
    for x in message.chars(){
        let message_binary = x as u8;
        message_bytes.push(message_binary);
    }
    message_bytes.push(0);
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


    let mut nl_count:u8 = 0;//number of new lines counted
    let mut create_letter_index:u8 = 0;//create_letter_index is the bit in the character that we are manipulating
    let mut message_index:u8 = 0;//message index is the current character that is being encoded
    let final_index:usize = message.len()+1;
    for byte in &mut bytes{
        //bytes is the array
        //byte is the current byte we are reading in from the array
    
        //checking if we are in pixel data
        if nl_count>=3 {
            if message_index == final_index as u8 {break;}
            let encode_char = message_bytes[message_index as usize];//encode_char is the character we are encoding
            let encode_char_val = check_bit(encode_char,create_letter_index);//value from encode char to compare

            //if the bit we checked is zero, else if it was on(odd)
            if encode_char_val==0{
                //current byte is odd
                if *byte %2 != 0{
                    *byte = unset_bit(*byte,7);
                }
            }
            else if encode_char > 0{
                //current byte is even
                if *byte%2==0{
                    *byte = set_bit(*byte,7);
                }
            }
            create_letter_index+=1;

            //done with a character
            if create_letter_index == 8{
                create_letter_index=0;//reset the index of the input character
                message_index+=1;//move on to the next character
            }
        }

        //checking if we have are accessing pixel data
        if *byte == 10 {nl_count+=1;}
    }

    //writing to standard output
    io::stdout().write(&bytes)?;
    Ok(bytes)
}

//sets a given bit at a given position, returns byte
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
        _ => panic!("Invalid index for setting a bit!")
    }
}
//unsets a given bit at a given position, returns byte
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
        _ => panic!("Invalid index for unsetting a bit!")
    }
}
//logically ands all but a given bit, returns byte
fn check_bit(byte: u8,position: u8)-> u8{
    match position{
        0 => byte & 0b1000_0000,
        1 => byte & 0b0100_0000,
        2 => byte & 0b0010_0000,
        3 => byte & 0b0001_0000,
        4 => byte & 0b0000_1000,
        5 => byte & 0b0000_0100,
        6 => byte & 0b0000_0010,
        7 => byte & 0b0000_0001,
        _ => panic!("Invalid index for checking a bit!")
    }
}
