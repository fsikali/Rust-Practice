/*
--- Attempting to use val after we've sent it down the channel
*/

use std::sync::mpsc; 
use std::thread; 

fn main() { 
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || { 
        let val = String::from("hi");
    });

    let received = rx.recv().unwrap(); 
    println!("Got {}", received());
}
