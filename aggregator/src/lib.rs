
pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Humble {

}

pub trait Dna {

}

pub trait Display {
    fn print_int() -> i32;

    fn some_function<T, U>(t: &T, u: &U) -> i32
        where T: Humble + Summary, U: Clone + Dna;
}

pub struct NewsArcticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArcticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// receiving an item that is implemented by Summary
pub fn do_summarize(item: &impl Summary) {
    println!("{}", item.summarize());
}

// this is the verbose way to do that
// we use the generics and say that the generic
// has the Summary implementaton
// **trait bound syntax**
pub fn old_do_summarize<T: Summary>(item: &T) {
    println!("{}", item.summarize())
}

// this syntax allows the user to pass two variables
// with different types that were implemented by Summary
pub fn summarize_two(item: &impl Summary, item2: &impl Summary) {

}

// this trait bound syntax guarantee that item and item2 
// will have the same type that were implemented by Summary
pub fn better_summarize_two<T: Summary, Y: Summary>(item: &T, item2: &Y) {

}

// specifying multiple traits
pub fn notify_with_display(item: &(impl Summary + Display)) {}

// using the trait bound syntax
pub fn old_notify_with_display<T: Summary + Display>(item: &T) {}

fn main() {

    let tweet = Tweet {
        username: String::from("me"),
        content: String::from("content very meaningfull"),
        reply: false,
        retweet: false
    };

}
