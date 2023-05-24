// use std::fs::File;
// use std::io::ErrorKind;
use std::fs;
// use std::io::{self, Read};
use std::io;

fn main() {
    // 1. verbose implementation
    // let greeting_file_result = File::open("hello.txt");
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };

    // 2. use `unwrap` to panic on error
    // let _greeting_file = File::open("hello.txt").unwrap();

    // 3. use `expect` to panic on error with better messages
    // let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    let username = read_username_from_file();
    println!("{:?}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    // 1. verbose implementation
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // 2. use `?` to propagate errors automatically
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // 3. use std functions
    fs::read_to_string("hello.txt")
}