use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn main() {
    // ? multiple producer single consumer
    // ? various senders but only one receiver
    // tx = transmitter
    // rx = receiver
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    // * now we have multiple senders and one receiver in practive

    thread::spawn(move || {
        let vals = vec![
            String::from("One"),
            String::from("Two"),
            String::from("Three"),
            String::from("Four"),
            String::from("Five")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // ? tx moved && takes ownership of val
        // ! error println!("Using val: {}", val);
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Uno"),
            String::from("Dos"),
            String::from("Tre"),
            String::from("Cuatro"),
            String::from("Cinco")
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
