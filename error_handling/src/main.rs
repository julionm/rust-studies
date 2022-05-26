use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f  = File::open("hello.txt");
    let fss = File::open("unavailable_file.txt")?;

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => panic!("Problem creating the file: {}", err)
            },
            other_error => {
                panic!("Problem opening the file: {}", other_error)
            }
        }
    };

    let a = File::open("hello2.txt").expect("Failed to open hello2.txt");

    match read_username_from_file(String::from("hello.txt")) {
        Ok(val) => println!("The username is: {}", val),
        Err(err) => panic!("An error occured: {}", err)
    };

    match read_username_from_file_reformed() {
        Ok(val) => println!("The username is: {}", val),
        Err(err) => panic!("An error occured: {}", err)
    };

    Ok(())
}

fn read_username_from_file(filename: String) -> Result<String, io::Error> {
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_reformed() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_reformed_second() -> Result<String, io::Error> {

    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}