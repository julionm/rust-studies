use std::{env, process, fs};
use std::error::Error;

use minigrep::Config;

fn main() {
    // panics if receive a not Unicode value
    let args: Vec<String> = env::args().collect();

    // * this err value is the one passed in Err call
    // * this closure is only executed if the Result is an Err
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // * as run doesn't return a value to unwrap
    // * we can use if let to return the error and exit
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
