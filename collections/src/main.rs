fn main() {

    let existing_vec = vec![1,2];

    let mut new_vec = Vec::new();
    new_vec.push(1); // type inferation Vec<i32>
    new_vec.push(2);
    new_vec.push(3);

    // accessing methods

    let third = &new_vec[2];

    println!("The third element is: {}", third);

    match existing_vec.get(2) {
        Some(third) => println!("The third element is: {}", third),
        None => println!("There is no third element")
    }
}
