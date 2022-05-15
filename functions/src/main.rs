fn main() {
    println!("The value of x is: {}", five());
    println!("Testing add function: {}", add(5, 3));
}

fn five () -> i32 {
    5
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
