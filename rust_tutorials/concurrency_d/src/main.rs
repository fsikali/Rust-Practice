/*
--- A thread with a closure that attempts to capture a reference to v from a main thread that drops v
*/

use std::thread; 

fn main() { 
    let v = vec![1, 2, 3]; 

    let handle = thread::spawn(|| { 
        println!("Here's a vector: {:?}", v); 
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
