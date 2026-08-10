use std::sync::{Arc, Mutex};
use std::{thread, vec};

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let data_clone = Arc::clone(&counter);

        let handle = thread::spawn(move ||{
            let mut num = data_clone.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}