use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Point;


fn main() {
    Point::hello_macro();
}
