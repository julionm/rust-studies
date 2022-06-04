use std::fs;
use std::env;
use std::error::Error;

// ! another thing i don't understand yet
// ? in the reading provided by the book, it tells
// ? it basically means this function will return an Error that
// ? implements the Error trait and dyn = dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

// rust needs to know from which argument the slices were made
// so we use lifetime to sign that
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // * iterators are a zero-cost abstraction
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results

}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    // ! didn't get the need of the annotation about static lifetime here
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // the first arg is defined by default 

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Insufficient parameters")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Insufficient parameters")
        };

        // ! extremely inefficient because clone creates a new allocation
        // ? instead is much more efficient to work with references
        // ? taking ownership of the iterators
        // let query = args[1].clone();
        // let filename = args[2].clone();

        // ! through the book this code seems to be wrong
        let case_sensitive = !env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}