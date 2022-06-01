use std::fs;
use std::error::Error;

// ! another thing i don't understand yet
// ? in the reading provided by the book, it tells
// ? it basically means this function will return an Error that
// ? implements the Error trait and dyn = dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With Text: \n{}", contents);

    Ok(())
}   

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    // ! didn't get the need of the annotation about static lifetime here
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}