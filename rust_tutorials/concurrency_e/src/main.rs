/*
--- Using the move keyword to force a closure to take ownership of the values it uses
*/

use std::thread; 

fn main() { 
    let v = vec![1, 2, 3]; 

    let handle = thread::spawn(move || { 
        println!("Here's a vector: {:?}", v);
    }); 

    handle.join().unwrap();
}

