/*
- The Box<T> type is ultimately defined as tuple struct with one element
- Defining a new function to match the new function defined on Box<T>
*/

// Example: Defining a MyBox<T> type

struct MyBox<T>(T); 

impl<T> MyBox<T> { 
    fn new(x: T) -> MyBox<T> { 
        MyBox(x)
    }
}

fn main() {}

