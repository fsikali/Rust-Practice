/*
- The code we would have to write if Rust didn't have deref coercion
*/

use std::ops::Deref; 

impl<T> Deref for MyBox<T> { 
    type Target = T; 

    fn deref(&self) -> &T { 
        &self.0
    }
}

struct MyBox<T>(T); 

impl<T> MyBox<T> { 
    fn new(x: T) -> MyBox<T> { 
        MyBox(x)
    }
}

fn hello(name: &str) { 
    println!("Hello, {name}!");
}

fn main() { 
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
