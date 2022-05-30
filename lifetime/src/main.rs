use std::fmt::Display;

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // borrowed value doesn´t live long enough
    // }

    // println!("PROBLEM: {}", r);

    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        result = longest(string1.as_str(), string2.as_str());
    }

    let result2 = longest_with_an_announcement(string1.as_str(), string2.as_str(), "anything");

    println!("The other result is: {}", result2);

    println!("The longest string is {}", result);

}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {

    if a.len() > b.len() {
        a
    } else {
        b
    }

}

// variation:
//* fn longest_with_an_announcement<'a, T: Display>(
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where 
    T: Display
{

    println!("ANNOUNCEMENT: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }

}
