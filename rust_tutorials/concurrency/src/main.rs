/*
--- Creating a new thread to print one thing while the main thread prints something else
*/

use std::thread;
use std::time::Duration;

fn main() { 
    thread::spawn(|| { 
        for i in 1..10 { 
            println!("hi number {} from the thread spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }); 

    for i in 1..5 { 
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
