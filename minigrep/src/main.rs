use std::{env, process};

use minigrep::Config;

fn main() {
    // * this err value is the one passed in Err call
    // * this closure is only executed if the Result is an Err
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // * as run doesn't return a value to unwrap
    // * we can use if let to return the error and exit
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
