/* 
- We can call the hello function with a string as an argument, such as hello("Rust");
  e.g. Deref coercion makes it possible to call a hello with a reference to a value of type 
       MyBox<String>
*/

// Example: Calling the hello with a reference to a MyBox<String> value, which works because
//          of deref coercion

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
    hello(&m);
}

