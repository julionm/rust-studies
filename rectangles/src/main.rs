#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // method of Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // associated function, because it not receives nor uses
    // self attribute
    fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 20
    };

    let sqr = Rectangle::create_square(20);

    println!("The rectangle is: {:?}", rect);
    println!("The square is {:?}", sqr);

    println!("Calculating the rect area using method: {}", rect.area());
    println!("Calculating the sqr area using method: {}", sqr.area());

    println!("The area of the rectangle is: {}", area_rectangle(&rect))
    println!("The area of the square is: {}", area_rectangle(&sqr))
}

// not used anymore becuase we've created the struct method
// to calculate the area for each Rectangle instance
fn area_rectangle(rect: &Rectangle) -> u32 { rect.height * rect.width }
