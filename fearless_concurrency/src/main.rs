use std::thread;
use std::time::Duration;

fn main() {

    let v: Vec<u32> = (4..14).collect();

    let handle = thread::spawn(move || {
        for i in v {
            println!("hi number {:?}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main");
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap(); // wait till this threads execution ends
}
