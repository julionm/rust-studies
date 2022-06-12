// ! THIS IS NOT MY CODE, HERE'S THE SAUCE 
// ! SOURCE: https://gist.github.com/wuct/b98f42dba91a8c08cc6ba2d820d94c1c

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let handle = thread::spawn(move || {
            let mut a_num = a.lock().unwrap();
            *a_num += 1;
            println!("Thread 1 holds a lock and starts waiting b lock");
            let mut b_num = b.lock().unwrap();
            *b_num += 1;
        });
        handles.push(handle);
    }
    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let handle = thread::spawn(move || {
            let mut b_num = b.lock().unwrap();
            *b_num += 1;
            println!("Thread 2 holds b lock and starts waiting a lock");
            let mut a_num = a.lock().unwrap();
            *a_num += 1;
            println!("Thread 2");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Done {}", *a.lock().unwrap()); // never reach here
}