/*
--- Exploring the API of Mutex<T> in a single-threaded context for simplicity
*/

use std::sync::Mutex; 

fn main() { 
    let m = Mutex::new(5);

    { 
        let mut num = m.lock().unwrap(); 
        *num = 6;
    }

    println!("m = {:?}", m);
}

