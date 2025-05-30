/*
--- What happens when we instead move handle.join() before the for loop in main,
*/

use std::thread;
use std::time::Duration;

fn main() { 
    let handle = thread::spawn(|| { 
        for i in 1..10 { 
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }); 

    handle.join().unwrap(); 

    for i in 1..5 { 
        println!("hi number {} from the main thread!", i); 
        thread::sleep(Duration::from_millis(1));
    }
}
