fn main() {
    println!("Hello, world!");
}

// ! if this was implemented using the Generics
// ! the components list would have all components of the same type
// ! cause, there'll be only a vector of ONE generic type

// ? but using trait bounds, it's possible to populate a vector with 
// ? any type that implements the Draw trait

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw SelectBox        
    }
}
