fn main() {
    let mut x = 99;
    let y = 42;
    let mut x_ref = &mut x;
    
    let two = (&mut *x_ref, y);
    
    *x_ref += 1;
    
    println!("Hi, {}", two.1);
}