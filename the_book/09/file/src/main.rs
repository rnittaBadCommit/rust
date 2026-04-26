use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(error) => panic!("Problem creating the file: {:?}", error),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                },
        },
    };
    let mut buf = [0u8; 1];
    let mut buf = Vec::new();
    buf.push('a' as u8);
    println!("{greeting_file:?}");
    println!("{:?}", greeting_file.read(&mut buf));
    // let x = loop {};
    // println!("{x}");

    // let x:!;
    // println!("{x}");
}