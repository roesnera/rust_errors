use std::fs::File;
use std::io::{self, Read};

fn main() {
//    let username = match read_username_from_file() {
//        Ok(string) => string,
//        Err(e) => panic!("{}",e),
//    };


    let username = read_username_from_file_qm().unwrap_or_else(|err| {
        panic!("{}",err)
    });
    println!("{username}");
}

// this is a crude way of error handling where we perform several operations that may result in an
// error and handle those cases using match statements
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// Rust provides a simpler way to do the exact same thing as above using the "?" operator
fn read_username_from_file_qm () -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
