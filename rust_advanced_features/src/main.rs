fn main() {
    let mut num = 6;
    // it's possible to create raw pointers in safe code
    // but it's not possible to derefer they
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // ! ERROR println!("{}", *r1);

    unsafe {
        println!("Works? {}", *r1);
        println!("Works? {}", *r2);
    }
}
