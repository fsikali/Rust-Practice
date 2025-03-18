/*
--- Receiving the value "hi" in the main thread and printing it
*/

use std::sync::mpsc;
use std::thread;

fn main() { 
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || { 
        let val = String::from("hi"); 
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); 
    println!("Got: {}", received);
}
