// Attempting to use a vector created by the main thread in another thread

use std::thread; 

fn main() { 
    let v = vec![1, 2, 3]; 

    let handle = thread::spawn(||  { 
        println!("Here's a vector: {:?}", v);
    }); 

    handle.join().unwrap();
}
