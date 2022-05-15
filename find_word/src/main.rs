use std::io;

fn main() {
    println!("Which index of the word you want to get?");

    println!("Provide text: ");

    let mut lorem_text = String::new();

    io::stdin()
        .read_line(&mut lorem_text)
        .expect("Failed getting index!");

    println!("\nProvide the index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed getting index!");

    let index_parsed = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    println!("\n\nWord is: {}",find_word(&lorem_text, index_parsed));
}

fn find_word(text: &String, index: u32) -> &str {

    let bytes_of = text.as_bytes();

    let mut word_counter = 0;
    let mut init_index = 0;

    for (i, &item) in bytes_of.iter().enumerate() {
        if item == b' ' {
            word_counter+=1;

            if word_counter == index - 1 {
                init_index = i;
            } else if word_counter == index {
                return &text[init_index..(i + 1)];
            }
        }
    }

    &text[..]

}
