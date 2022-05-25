use std::collections::HashMap;

enum CellValues {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {

    let existing_vec = vec![1,2];

    let mut new_vec = Vec::new();
    new_vec.push(1); // type inferation Vec<i32>
    new_vec.push(2);
    new_vec.push(3);

    // it's possible to store multiple types inside a vector
    // creating an enum to store those different values
    let row = vec![CellValues::Int(5), CellValues::Float(5.5), CellValues::Text(String::from("teste"))];

    // accessing methods

    let third = &new_vec[2];

    println!("The third element is: {}", third);

    match existing_vec.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element")
    }

    for i in &mut new_vec { // to get a mutable reference to array
        *i += 1; // we need to use the dereferencing operator to alter the value
    }

    for i in &new_vec { // to access an immutable reference to array
        println!("The value is: {}", i);
    }

    let s1 = String::from("test");
    let s2 = String::from("2");

    let s3 = s1 + &s2; 

    let b1 = String::from("tic");
    let b2 = String::from("tac");
    let b3 = String::from("toe");
    
    let b_final = format!("{}-{}-{}", b1, b2, b3);

    println!("{}", b_final);

    let string_to_slice = String::from("slice me bro");

    let sliced = &string_to_slice[0..4];
    println!("the sliced: {}", sliced);
    // after the above line, s1's value is moved
    // but s2 stays valid, when using + it uses a function add
    // so the move happends normally

    // for iterations over the characters
    for i in sliced.chars() {
        println!("{}", i)
    }

    // over bytes 
    for i in sliced.bytes() {
        println!("{}", i)
    }

    let mut hash_map_test = HashMap::new();

    hash_map_test.insert(String::from("Blue"), 10);

    let teams = vec![String::from("Team 1"), String::from("Team 2"), String::from("Team 3")];
    let team_scores: Vec<i32> = vec![10, 50, 60];

    // this logic creates a new hash map starting with 2 different vectors
    let scores_map: HashMap<_, _> = 
        teams.into_iter() // converts it to an mutable iterator type
             .zip( // pair two different iterators into one
                 team_scores.into_iter() // converts second array to an mutable iterator type
            ).collect(); // collect converts the result to the type of the variable
                         // it can be used to convert to many different collection types
    let first_value: Option<&i32> = scores_map.get("Team 1");

    for (key, value) in &scores_map {
        println!("The key value {} and the value {}", key, value);
    }
    

}
