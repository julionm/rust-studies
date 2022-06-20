use core::slice;
use std::error::Error;
use std::ops::Add;

extern "C" {
    fn abs(input: i32) -> i32;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { 
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

struct Human;

trait Wizard {
    fn fly(&self);
}

trait Pilot {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("Fly like the old movies")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wingardiu Leviosa")
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

static mut HELLO_WORLD: &str = "Hello, world!"; // * fixed address in memory

fn main() {
    let mut num = 6;
    // it's possible to create raw pointers in safe code
    // but it's not possible to derefer they
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // ! ERROR println!("{}", *r1);

    // ! ERROR danger_function();

    unsafe {
        println!("Works? {}", *r1);
        println!("Works? {}", *r2);

        danger_function();
    }

    let a = {
        println!("teste1");
        println!("teste2");

        1
    };

    println!("{a}");

    let mut v = vec![1,2,3,4,5,6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    // * a should be [1,2,3]
    // * b should be [4,5,6]

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    unsafe {
        println!("{HELLO_WORLD}"); 
        // when static variables are mutable, they can only be read inside an unsafe scope
    }


    let point1 = Point {
        x: 3,
        y: 4
    };

    let point2 = Point {
        x: 4,
        y: 6
    };

    let point3 = point1 + point2;

    println!("{:?}", point3);

    let h = Human;

    Wizard::fly(&h);
    Pilot::fly(&h);

    println!("A baby dog is called {}", <Dog as Animal>::baby_name());
}

// so the unsafe keyword creates an unsafe scope, in other words
// it can be used when creating a new scope to tells Rust this scope is
// unsafe for some reason

unsafe fn danger_function() {
    
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // * ptr points to the start of the values

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(
                ptr.add(mid),  // * changes the pointer's location to mid
                len - mid
            )
        )
    }
}

trait Anything {
    type T;
    type S;

    fn some_fn(&self) -> Option<Self::T>;

    fn other_gn(&self, a: &String) -> Result<Self::S, Box<dyn Error>>;
}

struct NiceStr {
    name: String
}

impl Anything for NiceStr {
    type T = u32;
    type S = char;

    fn some_fn(&self) -> Option<Self::T> {
        Some(3)
    }

    fn other_gn(&self, a: &String) -> Result<Self::S, Box<dyn Error>> {
        Ok('a')
    }

}
