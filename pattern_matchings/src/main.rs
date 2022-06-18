struct Point {
    x: i32,
    y: i32
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32)
}

enum Message {
    Quit,
    Hello { id: i32},
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color)
}

fn main() {
    let p = Point { x: 2, y: 3 };

    let Point { x: mut a, y: mut b } = p;

    let Point { x, y } = p;
    
    a += 1;
    b += 1;

    println!("{a} {b} {x} {y}");

    let msg = Message::Quit;

    match msg {
        Message::Quit => {
            println!("quiting")
        }
        Message::Move { x, y } => {
            println!("moving to {x} {y}")
        }
        Message::Write(text) => println!("my text {text}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("rgb colors are {r} {g} {b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("hsv colors are {h} {s} {v}"),
        Message::Hello {
            id: id_var @ vec![1, 3, 4]
        } => println!("o valor está entre 3 e 7, igual a: {id_var}"),
        Message::Hello {
            id
        } => println!("o id é {id}")
    }

    // * Destructuring
    let ((three, ten), Point { x: x1, y: y1 }) = ((3, 10), Point { x: 3, y: 4 });

    add(1, 2);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite the setting value!");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }

    let numbers = (2,4,8,16,32);

    match numbers {
        (first, ..) => {
            println!("checking only the first: {first}")
        }
        (first, _, third, _, fifth) => {
            println!("Some numbers {first} {third} {fifth}");
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        _ => {
            println!("none")
        }
    }

    let c = Some(5);
    let d = 10;
    let e = false;

    match c {
        Some(50) => println!("It's 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("default")
    }

    match d {
       4 | 5 | 10 if e => println!("nothing"),
       _ => println!("men...")

    }

}

fn add (_: i32, y: i32) {
    println!("it doesn't adds nothing, only shows {y}");
}
