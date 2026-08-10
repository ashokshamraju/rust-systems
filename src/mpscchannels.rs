use std::sync::mpsc;
use std::{println, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..5 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let result = i * i;
            tx_clone.send(result).unwrap();
        });
    }

    drop(tx);

    let mut sum = 0;
    for received in rx {
        sum += received;
    }

    println!("Total Sum is: {}",sum);
}