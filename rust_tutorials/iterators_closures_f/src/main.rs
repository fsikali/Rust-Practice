// Using move to force the closure for the thread to take ownership of list 

use std::thread; 

fn main() { 
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list); 

    thread::spawn(move || println!("From thread: {:?}", list))
          .join() 
          .unwrap();
}
